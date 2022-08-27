use crate::drivers::js::package::Package;
use crate::os::os::Os;
use crate::traits::generator::Generator;
use std::{collections::HashMap, fs::File, io::Write};

pub struct NodeGenerator {
    driver_options: HashMap<String, String>,
    package: Package,
}

impl NodeGenerator {
    pub fn new(driver_options: HashMap<String, String>) -> Self {
        let project_path = driver_options.get("path").unwrap();

        Self {
            package: Package::new(format!("{}/package.json", project_path)).unwrap(),
            driver_options,
        }
    }

    fn dependencies(&self) -> &serde_json::Map<String, serde_json::Value> {
        self.package.data()["dependencies"].as_object().unwrap()
    }

    fn find_os_packages(&self) -> Vec<String> {
        let mut os_packages = vec![];

        // later change apt-get / apk commands to match with container os
        // let os = self.driver_options.get("os")
        //     .unwrap_or(&"ubuntu".to_owned())
        //     .parse::<Os>()
        //     .unwrap();

        // let _ = match os {
        //     Os::Ubuntu => unimplemented!(),
        //     Os::Alpine => unimplemented!(),
        // };

        for (key, _) in self.dependencies().iter() {
            let os_package = match key.as_str() {
                "@grpc/grpc-js" | "@grpc/proto-loader" => {
                    Some("libprotobuf-dev protobuf-compiler".to_owned())
                }
                _ => None,
            };

            if let Some(os_package) = os_package {
                if !os_packages.contains(&os_package) {
                    os_packages.push(os_package);
                }
            }
        }

        os_packages
    }

    pub fn find_images(&self) -> HashMap<String, String> {
        let mut images: HashMap<String, String> = HashMap::new();

        for (key, value) in self.dependencies().iter() {
            let image = match key.as_str() {
                "ioredis" | "redis" => Some("redis".to_owned()),
                "mongodb" | "mongoose" => Some("mongodb".to_owned()),
                _ => None,
            };

            if let Some(image) = image {
                images.insert(image, value.to_string());
            }
        }

        images
    }
}

impl Generator for NodeGenerator {
    fn generate(&self) {
        let project_path = self.driver_options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let version = self.package.find_node_version();

        let os_packages = self.find_os_packages();

        dockerfile_contents.push_str(
            format!(
                "
            FROM node:{}
            WORKDIR /app
            RUN apt-get update
            RUN apt-get install -y {}
            COPY package*.json tsconfig.json ./
            RUN npm i
            COPY . .
            ",
                version,
                os_packages.join(" ")
            )
            .as_str(),
        );

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => println!("Dockerfile generated at: {}", project_path),
            Err(_) => unimplemented!(),
        }
    }
}