use crate::drivers::js::package::Package;
use crate::traits::generator::Generator;
use std::{fs::File, io::Write};

pub struct NodeGenerator {}

impl NodeGenerator {
    fn generate_package(package: Package) -> String {
        let node_version = package.data()["engines"]["node"]
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

impl Generator for NodeGenerator {
    fn generate(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let package = Package::new(format!("{}/package.json", project_path)).unwrap();

        dockerfile_contents.push_str(Self::generate_package(package).as_str());

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }
}
