use crate::drivers::DriverGenerator;
// use crate::os::os::Os;
use crate::{compose::Compose, drivers::js::package::Package};
use serde_json::json;
use std::io::{BufReader, Read};
use std::{collections::HashMap, fs::File, io::Write};

pub struct JSGenerator {
    options: HashMap<String, String>,
    package: Package,
}

impl JSGenerator {
    pub fn new(options: HashMap<String, String>) -> Self {
        let project_path = options.get("path").unwrap();

        Self {
            package: Package::new(format!("{}/package.json", project_path)).unwrap(),
            options,
        }
    }

    fn dependencies(&self) -> &serde_json::Map<String, serde_json::Value> {
        self.package.data()["dependencies"].as_object().unwrap()
    }

    fn dev_dependencies(&self) -> &serde_json::Map<String, serde_json::Value> {
        self.package.data()["devDependencies"].as_object().unwrap()
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
                "@grpc/grpc-js" | "@grpc/proto-loader" | "protobufjs" => {
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
}

impl DriverGenerator for JSGenerator {
    fn generate(&self) {
        let project_path = self.options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let version = self.package.find_node_version();

        let os_packages = self.find_os_packages();

        dockerfile_contents.push_str(
            format!(
                "FROM node:{}
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

    fn add_to_ignore(&self, ignore: &mut String) {
        ignore.push_str("\n\n# app\nnode_modules")
    }

    fn find_images(&self) -> HashMap<String, String> {
        let mut images: HashMap<String, String> = HashMap::new();

        // @TODO: match with regex or something else instead of hard coded strings

        for (key, value) in self.dev_dependencies().iter() {
            let image = match key.as_str() {
                "prisma" => {
                    let prisma_schema = File::open(format!(
                        "{}/prisma/schema.prisma",
                        self.options.get("path").unwrap()
                    )).expect("You have `prisma` in your devDependencies yet you don't have prisma/schema.prisma file.");

                    let mut buf_reader = BufReader::new(prisma_schema);
                    let mut contents = String::new();
                    buf_reader.read_to_string(&mut contents).unwrap();

                    let re = regex::Regex::new(r#"datasource db.*\n.*provider = "(.*)""#).unwrap();

                    let captures = re.captures(&contents).expect("You have `prisma` in your devDependencies yet you don't have `datasource db` definition in schema.prisma file.");

                    match &captures[1] {
                        // "postgresql" | "mysql" => Some(captures[1].to_owned()),
                        "mongodb" => Some("mongo".to_owned()),
                        // "cockroachdb" => Some("cockroachdb/cockroach".to_owned()),
                        // @TODO: for sqlite i have to add it to os packages
                        _ => None,
                    }
                }
                _ => None,
            };

            if let Some(image) = image {
                images.insert(image, value.to_string());
            }
        }

        for (key, value) in self.dependencies().iter() {
            let image = match key.as_str() {
                "ioredis" | "redis" => Some("redis".to_owned()),
                "mongodb" | "mongoose" => Some("mongo".to_owned()),
                "@elastic/elasticsearch" => Some("elasticsearch".to_owned()),
                _ => None,
            };

            if let Some(image) = image {
                images.insert(image, value.to_string());
            }
        }

        println!("{:?}", images);

        images
    }
}

impl Compose for JSGenerator {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        let project_path = self.options.get("path").unwrap();
        let images = &self.find_images();
        let depends_on = images.keys().collect::<Vec<&String>>();

        HashMap::from([(
            "services",
            json!({
                "app": {
                    "build": ".",
                    "image": format!("{}-image", project_path),
                    "volumes": [
                        "./:/app",
                        "/app/node_modules"
                    ],
                    "ports": [
                        "8000:8000"
                    ],
                    "env_file": "./.env",
                    "depends_on": depends_on
                }
            }),
        )])
    }
}
