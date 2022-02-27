use super::composer::Composer;
use std::fs::File;

pub struct Generator { }

impl Generator {
    pub fn run(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path)).expect("Dockerfile can't be created.");

        // used extensions can be found in composer.json's require object like "ext-pdo", "ext-mongo"
        let composer = Composer::new(format!("{}/composer.json", project_path), 2);

        let mut version = composer.data()["require"]["php"].as_str().unwrap_or_else(|| "latest");

        println!("PHP Version: {}", version);
                         
        let mut dockerfile_contents = b"FROM php{}-fpm";
    }
}