use crate::drivers::js::package::Package;
use crate::traits::generator::Generator;
use std::{fs::File, io::Write, collections::HashMap};

pub struct NodeGenerator<'a> {
    project_path: &'a String,
    package: Package
}

impl<'a> NodeGenerator<'a> {
    pub fn new(project_path: &'a String) -> Self {
        Self {
            project_path,
            package: Package::new(format!("{}/package.json", project_path)).unwrap()
        }
    }

    fn generate_package(&self) -> String {
        let node_version = self.package.data()["engines"]["node"]
            .as_str()
            .unwrap_or("16")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>();

        // find node version from package.json or somehow if can't be found by package.json
        format!(
            "
            FROM node:{}-alpine
            WORKDIR /app
            COPY package*.json tsconfig.json ./
            RUN npm i
            COPY . .
            ",
            node_version
        )
    }
}

impl Generator for NodeGenerator<'_> {
    fn generate(&self) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", self.project_path)).expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        dockerfile_contents.push_str(self.generate_package().as_str());

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", self.project_path),
            Err(_) => unimplemented!(),
        }
    }

    fn find_images(&self) -> HashMap<String, String> {
        let mut images: HashMap<String, String> = HashMap::new();

        for (key, value) in self.package.data()["dependencies"].as_object().unwrap().iter() {
            let image = match key.as_str() {
                "ioredis" | "redis" => Some("redis".to_owned()),
                "mongodb" | "mongoose" => Some("mongodb".to_owned()),
                _ => None
            };

            if let Some(image) = image {
                images.insert(image, value.to_string());
            }
        }

        images
    }
}
