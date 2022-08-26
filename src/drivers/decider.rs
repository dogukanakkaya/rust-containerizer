use std::collections::HashMap;
use super::driver::Driver;
use super::generator::Generator;
use super::php::generator::PHPGenerator;
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
        
        match driver {
            Driver::PHP => PHPGenerator::generate(project_path),
            Driver::NodeJS => unimplemented!(),
            _ => unimplemented!()
        };
    }

    pub fn driver_options(&self) -> &HashMap<String, String> {
        &self.driver_options
    }
}