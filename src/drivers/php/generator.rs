use super::composer::Composer;
use crate::{
    compose::Compose,
    drivers::{js::package::Package, DriverGenerator},
    images::Image,
};
use serde_json::json;
use std::{collections::HashMap, fs::File, io::Write};

pub struct PHPGenerator {
    options: HashMap<String, String>,
    composer: Composer,
    package: Result<Package, String>,
    os_packages: Vec<String>,
    images: Vec<String>,
    extensions: Vec<String>,
}

impl PHPGenerator {
    pub fn new(options: HashMap<String, String>) -> Self {
        let project_path = options.get("path").unwrap();

        Self {
            composer: Composer::new(format!("{}/composer.json", project_path)).unwrap(),
            package: Package::new(format!("{}/package.json", project_path)),
            options,
            os_packages: vec![],
            images: vec![],
            extensions: vec![],
        }
    }
}

impl DriverGenerator for PHPGenerator {
    fn collect(&mut self) {
        let all_dependencies = self.composer.all_dependencies();

        for (key, value) in all_dependencies.iter() {
            if key.starts_with("ext-") && *value == "*" {
                self.extensions.push(key[4..].to_owned());
            }

            // @TODO: match with regex or something else instead of hard coded strings
            match key.as_str() {
                "phpredis/phpredis" | "predis/predis" => self.images.push("redis".to_owned()),
                "mongodb" | "mongoose" => self.images.push("mongo".to_owned()),
                "elasticsearch/elasticsearch" => self.images.push("elasticsearch".to_owned()),
                _ => {}
            };
        }
    }

    fn generate(&self) {
        let project_path = self.options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        dockerfile_contents.push_str(
            format!(
                "FROM php:{}-fpm
WORKDIR /app
RUN apt-get update && apt-get install -y g++ git
RUN docker-php-ext-install {}
            ",
                self.composer.find_php_version(),
                self.extensions.join(" ")
            )
            .as_str(),
        );

        let package = self.package.as_ref();

        match package {
            Ok(pck) => dockerfile_contents.push_str(
                format!(
                    "
RUN curl -fsSL https://deb.nodesource.com/setup_{}.x | bash -
RUN apt-get install -y nodejs
COPY composer.json composer.lock symfony.lock ./
RUN composer install
COPY package*.json .
RUN npm i
                ",
                    pck.find_node_version()
                )
                .as_str(),
            ),
            _ => dockerfile_contents.push_str(
                "
COPY composer.json composer.lock symfony.lock ./
RUN composer install
            ",
            ),
        }

        dockerfile_contents.push_str("COPY . .");

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }

    fn add_to_ignore(&self, ignore: &mut String) {
        ignore.push_str("\n\n# app\nvendor")
    }

    fn images(&self) -> &Vec<String> {
        self.images.as_ref()
    }

    fn os_packages(&self) -> &Vec<String> {
        self.os_packages.as_ref()
    }
}

impl Compose for PHPGenerator {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        let project_path = self.options.get("path").unwrap();
        let depends_on = Image::filter_implemented_images(&self.images);

        HashMap::from([(
            "services",
            json!({
                "app": {
                    "build": ".",
                    "image": format!("{}-image", project_path),
                    "volumes": [
                        "./:/app",
                        "/app/vendor"
                    ],
                    "ports": [
                        "8000:8000"
                    ],
                    "env_file": "./.env",
                    "depends_on": depends_on
                }
            }),
        )])
    }
}
