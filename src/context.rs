use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::traits::generator::Generator;
use crate::drivers::driver::Driver;
use crate::drivers::php::generator::PHPGenerator;
use crate::drivers::js::node::generator::NodeGenerator;
use dotenv;
use serde_json::json;

#[derive(Debug)]
pub struct Context {
    driver_options: HashMap<String, String>
}

impl From<HashMap<String, String>> for Context {
    fn from(driver_options: HashMap<String, String>) -> Self {
        Self {
            driver_options
        }
    }
}

impl Context {
    pub fn decide(&self) {
        let driver = self.driver_options.get("driver").expect("Option driver is missing. Did you forget to add --driver option?")
            .parse::<Driver>()
            .unwrap();
        let project_path = self.driver_options.get("path").expect("Option path is missing. Did you forget to add --path option?");

        // used extensions can be guessed from .env file
        dotenv::from_filename(format!("{}/.env", project_path)).expect(&format!(".env file is not exists in path {}", project_path));

        let mut docker_compose = File::create(format!("{}/docker-compose.yaml", project_path)).expect("docker-compose.yaml can't be created.");
        let mut docker_compose_contents = json!({
            "version": "3.8",
            "services": { }
        });
        
        let (images, _) = match driver {
            Driver::PHP => {
                let php_generator = PHPGenerator::new(self.driver_options.clone());
                php_generator.generate();

                (
                    php_generator.find_images(),
                    1
                )
            },
            Driver::NodeJS => {
                let node_generator = NodeGenerator::new(self.driver_options.clone());
                node_generator.generate();

                docker_compose_contents["services"]
                    .as_object_mut()
                    .unwrap()
                    .insert("node".to_owned(), json!({
                        "build": ".",
                        "image": format!("{}-image", project_path),
                        "ports": [
                            "8000:8000"
                        ],
                        "env_file": "./.env",
                        "depends_on": []
                    }));
                (
                    node_generator.find_images(),
                    1
                )
            },
            _ => unimplemented!()
        };

        println!("images: {:?}", images);
        let yaml = serde_yaml::to_string::<serde_json::Value>(&docker_compose_contents).unwrap();

        match docker_compose.write_all(yaml.as_bytes()) {
            Ok(()) => println!("docker-compose.yaml generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }
}