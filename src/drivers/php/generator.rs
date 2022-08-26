use super::composer::Composer;
use crate::drivers::js::package::Package;
use std::{fs::File, io::Write};

const SUPPORTED_IMAGES: [&'static str; 4] = ["elasticsearch", "redis", "mysql", "postgresql"];

pub struct Generator { }

impl Generator {
    pub fn run(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path)).expect("Dockerfile can't be created.");

        let composer = Composer::new(format!("{}/composer.json", project_path)).unwrap();

        let package = Package::new(format!("{}/package.json", project_path));

        let php_version = composer.data()["require"]["php"].as_str().unwrap_or_else(|| "latest").chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

        let mut dockerfile_contents = format!("FROM php:{}-fpm\nWORKDIR /var/www/php\nRUN apt-get update\n", php_version);

        let require = &composer.data()["require"].as_object().unwrap();

        // find used extensions in composer.json's require object like "ext-pdo", "ext-mongo"
        let mut extensions = vec![];

        for req in require.iter() {
            let (key, value) = req;
            if key.starts_with("ext-") && value == "*" {
                extensions.push(&key[4..]);
            }
        }

        dockerfile_contents.push_str(format!("RUN docker-php-ext-install {}\n", extensions.join(" ")).as_str());
         
        // install nodejs if package.json exists
        match package {
            Ok(pck) => {
                let node_version = pck.data()["engines"]["node"].as_str().unwrap_or_else(|| "16").chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

                // find node version from package.json or somehow if can't be found by package.json
                dockerfile_contents.push_str(format!(
                    "RUN curl -fsSL https://deb.nodesource.com/setup_{}.x | bash -\nRUN apt-get-install -y nodejs",
                    &node_version[..node_version.find(".").unwrap()]
                ).as_str());
            },
            _ => {}
        }

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!()
        }
    }
}