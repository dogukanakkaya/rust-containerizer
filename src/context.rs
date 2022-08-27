use crate::drivers::driver::Driver;
use crate::drivers::js::generator::JSGenerator;
use crate::drivers::php::generator::PHPGenerator;
use crate::traits::{compose::Compose, generator::Generator};
use dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Context {
    driver_options: HashMap<String, String>,
}

impl From<HashMap<String, String>> for Context {
    fn from(driver_options: HashMap<String, String>) -> Self {
        Self { driver_options }
    }
}

impl Context {
    pub fn decide(&self) {
        let driver = self
            .driver_options
            .get("driver")
            .expect("Option driver is missing. Did you forget to add --driver option?")
            .parse::<Driver>()
            .unwrap();
        let project_path = self
            .driver_options
            .get("path")
            .expect("Option path is missing. Did you forget to add --path option?");

        // used extensions can be guessed from .env file
        dotenv::from_filename(format!("{}/.env", project_path))
            .expect(&format!(".env file is not exists in path {}", project_path));

        let mut docker_compose = File::create(format!("{}/docker-compose.yaml", project_path))
            .expect("docker-compose.yaml can't be created.");
        let mut docker_compose_contents = json!({
            "version": "3.8",
            "services": { }
        });

        let (images, _) = match driver {
            Driver::PHP => {
                let generator = PHPGenerator::new(self.driver_options.clone());
                generator.generate();

                (generator.find_images(), 1)
            }
            Driver::JS => {
                let generator = JSGenerator::new(self.driver_options.clone());
                generator.generate();

                let compose_definition = generator.find_compose_definition();
                JSGenerator::add_to_compose(compose_definition, &mut docker_compose_contents);

                (generator.find_images(), 1)
            }
            _ => unimplemented!(),
        };

        let yaml = serde_yaml::to_string::<serde_json::Value>(&docker_compose_contents).unwrap();

        match docker_compose.write_all(yaml.as_bytes()) {
            Ok(()) => println!("docker-compose.yaml generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }
}
