use crate::compose::mongodb::MongoDB;
use crate::compose::redis::Redis;
use crate::drivers::driver::Driver;
use crate::drivers::js::generator::JSGenerator;
use crate::drivers::php::generator::PHPGenerator;
use crate::images::image::Image;
use crate::traits::Driver as DriverTrait;
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
    pub fn exec(self) {
        let driver = self
            .driver_options
            .get("driver")
            .expect("Option driver is missing. Did you forget to add --driver option?")
            .parse::<Driver>()
            .unwrap();
        let compose = self
            .driver_options
            .get("compose")
            .unwrap_or(&"true".to_owned())
            .to_owned();
        let project_path = self
            .driver_options
            .get("path")
            .expect("Option path is missing. Did you forget to add --path option?");

        // used extensions can be guessed from .env file
        dotenv::from_filename(format!("{}/.env", project_path))
            .expect(&format!(".env file is not exists in path {}", project_path));

        let generator: Box<dyn DriverTrait> = match driver {
            Driver::PHP => Box::new(PHPGenerator::new(self.driver_options.clone())),
            Driver::JS => Box::new(JSGenerator::new(self.driver_options.clone())),
        };

        generator.generate();

        if compose == "true" {
            let mut docker_compose = File::create(format!("{}/docker-compose.yaml", project_path))
                .expect("docker-compose.yaml can't be created.");
            let mut docker_compose_contents = json!({
                "version": "3.8",
                "services": { }
            });

            generator.add_to_compose(&mut docker_compose_contents);

            for (image, _) in generator.find_images() {
                match image.parse::<Image>().unwrap() {
                    Image::Redis => {
                        let redis = Redis::new();

                        redis.add_to_compose(&mut docker_compose_contents);
                    }
                    Image::MongoDB => {
                        let mongodb = MongoDB::new();

                        mongodb.add_to_compose(&mut docker_compose_contents);
                    }
                }
            }

            let yaml =
                serde_yaml::to_string::<serde_json::Value>(&docker_compose_contents).unwrap();

            match docker_compose.write_all(yaml.as_bytes()) {
                Ok(()) => println!("docker-compose.yaml generated at: {}", project_path),
                Err(_) => unimplemented!(),
            }
        }
    }
}
