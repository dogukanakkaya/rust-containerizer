use super::composer::Composer;
use crate::{
    compose::Compose,
    drivers::{js::package::Package, DriverGenerator},
};
use serde_json::json;
use std::{collections::HashMap, fs::File, io::Write};

pub struct PHPGenerator {
    driver_options: HashMap<String, String>,
    composer: Composer,
    package: Result<Package, String>,
}

impl PHPGenerator {
    pub fn new(driver_options: HashMap<String, String>) -> Self {
        let project_path = driver_options.get("path").unwrap();

        Self {
            composer: Composer::new(format!("{}/composer.json", project_path)).unwrap(),
            package: Package::new(format!("{}/package.json", project_path)),
            driver_options,
        }
    }

    fn dependencies(&self) -> &serde_json::Map<String, serde_json::Value> {
        self.composer.data()["require"].as_object().unwrap()
    }

    fn find_extensions(&self) -> Vec<&str> {
        let mut extensions = vec![];

        for (key, value) in self.composer.data()["require"].as_object().unwrap().iter() {
            if key.starts_with("ext-") && value == "*" {
                extensions.push(&key[4..]);
            }
        }

        extensions
    }

    // @TODO: implement creation .dockerignore
//     fn ignore() {
//         "
// ##> Docker <##
// Dockerfile
// .dockerignore
// docker-compose*
// ##/> Docker <##

// ##> VCS <##
// .git
// .gitignore
// ##/> VCS <##

// ##> App <##
// node_modules
// ##/> App <##
//         ";
//     }
}

impl DriverGenerator for PHPGenerator {
    fn generate(&self) {
        let project_path = self.driver_options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        dockerfile_contents.push_str(
            format!(
                "
            FROM php:{}-fpm
            WORKDIR /app
            RUN apt-get update && apt-get install -y g++ git
            RUN docker-php-ext-install {}
            ",
                self.composer.find_php_version(),
                self.find_extensions().join(" ")
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

    fn find_images(&self) -> HashMap<String, String> {
        let mut images: HashMap<String, String> = HashMap::new();

        for (key, value) in self.dependencies().iter() {
            // @TODO: match with regex or something else instead of hard coded strings
            let image = match key.as_str() {
                "phpredis/phpredis" | "predis/predis" => Some("redis".to_owned()),
                "mongodb" | "mongoose" => Some("mongo".to_owned()),
                "elasticsearch/elasticsearch" => Some("elasticsearch".to_owned()),
                _ => None,
            };

            if let Some(image) = image {
                images.insert(image, value.to_string());
            }
        }

        images
    }
}

impl Compose for PHPGenerator {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        let project_path = self.driver_options.get("path").unwrap();
        let images = &self.find_images();
        let depends_on = images.keys().collect::<Vec<&String>>();

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
