use super::composer::Composer;
use crate::drivers::{js::package::Package, generator::Generator};
use std::{fs::File, io::Write};

const SUPPORTED_IMAGES: [&'static str; 4] = ["elasticsearch", "redis", "mysql", "postgresql"];

pub struct PHPGenerator { }

impl PHPGenerator {
    fn generate_composer(composer: Composer) -> String {
        let php_version = composer.data()["require"]["php"].as_str().unwrap_or_else(|| "latest").chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

        // find used extensions in composer.json's require object like "ext-pdo", "ext-mongo"
        let mut extensions = vec![];

        for require in composer.data()["require"].as_object().unwrap().iter() {
            let (key, value) = require;
            
            if key.starts_with("ext-") && value == "*" {
                extensions.push(&key[4..]);
            }
        }

        format!(
            "
            FROM php:{}-fpm\n
            WORKDIR /var/www/php\n
            RUN apt-get update\n
            RUN docker-php-ext-install {}\n
            ",
            php_version,
            extensions.join(" ")
        )
    }

    fn generate_package(package: Package) -> String {  
        let node_version = package.data()["engines"]["node"].as_str().unwrap_or_else(|| "16").chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

        // find node version from package.json or somehow if can't be found by package.json
        format!(
            "
            RUN curl -fsSL https://deb.nodesource.com/setup_{}.x | bash -\nRUN apt-get-install -y nodejs
            ",
            &node_version[..node_version.find(".").unwrap()]
        )
    }
}

impl Generator for PHPGenerator {
    fn generate(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path)).expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let composer = Composer::new(format!("{}/composer.json", project_path)).unwrap();
        let package = Package::new(format!("{}/package.json", project_path));

        dockerfile_contents.push_str(Self::generate_composer(composer).as_str());

        match package {
            Ok(pck) => dockerfile_contents.push_str(Self::generate_package(pck).as_str()),
            _ => ()
        }

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!()
        }
    }
}