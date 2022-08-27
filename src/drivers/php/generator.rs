use super::composer::Composer;
use crate::drivers::js::package::Package;
use crate::traits::generator::Generator;
use std::collections::HashMap;
use std::{fs::File, io::Write};

pub struct PHPGenerator<'a> {
    project_path: &'a String
    // lines: HashMap<i8, String>,
}

impl<'a> PHPGenerator<'a> {
    pub fn new(project_path: &'a String) -> Self {
        Self { project_path }
    }

    fn generate_composer(composer: Composer) -> String {
        let php_version = composer.data()["require"]["php"]
            .as_str()
            .unwrap_or("latest")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>();

        // find used extensions in composer.json's require object like "ext-pdo", "ext-mongo"
        let mut extensions = vec![];

        for require in composer.data()["require"].as_object().unwrap().iter() {
            let (key, value) = require;

            if key.starts_with("ext-") && value == "*" {
                extensions.push(&key[4..]);
            }

            // check for known packages like elasticsearch/elasticsearch and create a compose file for those
        }

        // self.lines.insert(1, format!(
        //     "
        //     FROM php:{}-fpm
        //     WORKDIR /var/www/php
        //     RUN apt-get update
        //     RUN docker-php-ext-install {}
        //     COPY composer.json composer.lock symfony.lock ./
        //     ",
        //     php_version,
        //     extensions.join(" ")
        // ));
        // self.lines.insert(3, format!(
        //     "
        //     RUN composer install
        //     COPY . .
        //     "
        // ));

        format!(
            "
            FROM php:{}-fpm
            WORKDIR /var/www/php
            RUN apt-get update
            RUN docker-php-ext-install {}
            COPY composer.json composer.lock symfony.lock ./
            RUN composer install
            COPY . .
            ",
            php_version,
            extensions.join(" ")
        )
    }

    fn generate_package(package: Package) -> String {
        let node_version = package.data()["engines"]["node"]
            .as_str()
            .unwrap_or("16")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>();

        // find node version from package.json or somehow if can't be found by package.json
        format!(
            "
            RUN curl -fsSL https://deb.nodesource.com/setup_{}.x | bash -
            RUN apt-get-install -y nodejs
            COPY package*.json .
            RUN npm i
            COPY . .
            ",
            &node_version[..node_version.find(".").unwrap()]
        )
    }
}

impl Generator for PHPGenerator<'_> {
    fn generate(&self) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", self.project_path)).expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let composer = Composer::new(format!("{}/composer.json", self.project_path)).unwrap();
        let package = Package::new(format!("{}/package.json", self.project_path));

        // let php_generator = Self::new();

        dockerfile_contents.push_str(Self::generate_composer(composer).as_str());

        match package {
            Ok(pck) => dockerfile_contents.push_str(Self::generate_package(pck).as_str()),
            _ => (),
        }

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", self.project_path),
            Err(_) => unimplemented!(),
        }
    }

    fn find_images(&self) -> Vec<String> {
        vec![String::new()]
    }
}
