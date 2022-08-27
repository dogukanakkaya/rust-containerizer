use std::collections::HashMap;
use crate::traits::generator::Generator;
use super::driver::Driver;
use super::php::generator::PHPGenerator;
use super::js::node::generator::NodeGenerator;
use dotenv;

#[derive(Debug)]
pub struct Decider {
    driver_options: HashMap<String, String>
}

impl From<HashMap<String, String>> for Decider {
    fn from(driver_options: HashMap<String, String>) -> Self {
        Self {
            driver_options
        }
    }
}

impl Decider {
    pub fn decide(&self) {
        let driver: Driver = self.driver_options.get("driver").expect("Option driver is missing. Did you forget to add --driver option?").parse().unwrap();
        let project_path = self.driver_options.get("path").expect("Option path is missing. Did you forget to add --path option?");

        // used extensions can be guessed from .env file
        dotenv::from_filename(format!("{}/.env", project_path)).expect(&format!(".env file is not exists in path {}", project_path));
        
        let (images, i) = match driver {
            Driver::PHP => {
                let php_generator = PHPGenerator::new(project_path);
                php_generator.generate();

                (
                    php_generator.find_images(),
                    1
                )
            },
            Driver::NodeJS => {
                let node_generator = NodeGenerator::new(project_path);
                node_generator.generate();

                (
                    node_generator.find_images(),
                    1
                )
            },
            _ => unimplemented!()
        };

        println!("images: {:?}", images);
    }

    pub fn driver_options(&self) -> &HashMap<String, String> {
        &self.driver_options
    }
}