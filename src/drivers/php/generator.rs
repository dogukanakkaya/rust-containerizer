use super::composer::Composer;
use crate::drivers::js::package::Package;
use crate::traits::generator::Generator;
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

    fn find_extensions(&self) -> Vec<&str> {
        let mut extensions = vec![];

        for (key, value) in self.composer.data()["require"].as_object().unwrap().iter() {
            if key.starts_with("ext-") && value == "*" {
                extensions.push(&key[4..]);
            }
        }

        extensions
    }

    pub fn find_images(&self) -> HashMap<String, String> {
        let mut images: HashMap<String, String> = HashMap::new();

        images
    }
}

impl Generator for PHPGenerator {
    fn generate(&self) {
        let project_path = self.driver_options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        dockerfile_contents.push_str(
            format!(
                "
            FROM php:{}-fpm
            WORKDIR /var/www/php
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
                COPY package*.json .
                RUN npm i
                ",
                    pck.find_node_version()
                )
                .as_str(),
            ),
            _ => (),
        }

        dockerfile_contents.push_str(
            format!(
                "
            COPY composer.json composer.lock symfony.lock ./
            RUN composer install
            COPY . .
            "
            )
            .as_str(),
        );

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }
}
