use super::composer::Composer;
use crate::drivers::js::package::Package;
use std::{fs::File, io::Write};

pub struct Generator { }

impl Generator {
    pub fn run(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path)).expect("Dockerfile can't be created.");

        // used extensions can be found in composer.json's require object like "ext-pdo", "ext-mongo"
        let composer = Composer::new(format!("{}/composer.json", project_path));

        let php_version = composer.data()["require"]["php"].as_str().unwrap_or_else(|| "latest").chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

        let mut dockerfile_contents = format!("FROM php:{}-fpm\nWORKDIR /var/www/php\nRUN apt-get update\n", php_version);
        
        let package = Package::new(format!("{}/package.json", project_path));

        // install nodejs if package.json exists
        match package {
            Ok(_) => {
                // find node version from package.json or somehow if can't be found by package.json
                dockerfile_contents.push_str("RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -\nRUN apt-get-install -y nodejs");
            },
            _ => {}
        }

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!()
        }
    }
}