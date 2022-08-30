use crate::drivers::DriverGenerator;
// use crate::os::os::Os;
use crate::{compose::Compose, drivers::js::package::Package};
use serde_json::json;
use std::io::{BufReader, Read};
use std::{collections::HashMap, fs::File, io::Write};

pub struct JSGenerator {
    options: HashMap<String, String>,
    package: Package,
    os_packages: Vec<String>,
    images: Vec<String>,
}

impl JSGenerator {
    pub fn new(options: HashMap<String, String>) -> Self {
        let project_path = options.get("path").unwrap();

        Self {
            package: Package::new(format!("{}/package.json", project_path)).unwrap(),
            options,
            os_packages: vec![],
            images: vec![],
        }
    }
}

impl DriverGenerator for JSGenerator {
    fn collect(&mut self) {
        let all_dependencies = self.package.all_dependencies();

        // @TODO: match with regex or something else instead of hard coded strings
        for (key, _value) in all_dependencies.iter() {
            match key.as_str() {
                "ioredis" | "redis" => self.images.push("redis".to_owned()),
                "mongodb" | "mongoose" => self.images.push("mongo".to_owned()),
                "@elastic/elasticsearch" => self.images.push("elasticsearch".to_owned()),
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
                        "postgresql" | "mysql" => self.images.push(captures[1].to_owned()),
                        "mongodb" => self.images.push("mongo".to_owned()),
                        "cockroachdb" => self.images.push("cockroachdb/cockroach".to_owned()),
                        "sqlite" => self.os_packages.push("sqlite3 libsqlite3-dev".to_owned()),
                        _ => {}
                    }
                }
                "@grpc/grpc-js" | "@grpc/proto-loader" | "protobufjs" => self
                    .os_packages
                    .push("libprotobuf-dev protobuf-compiler".to_owned()),
                _ => {}
            };
        }
    }

    fn generate(&self) {
        let project_path = self.options.get("path").unwrap();

        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path))
            .expect("Dockerfile can't be created.");
        let mut dockerfile_contents = String::new();

        let version = self.package.find_node_version();

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
                self.os_packages.join(" ")
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

    fn images(&self) -> &Vec<String> {
        self.images.as_ref()
    }

    fn os_packages(&self) -> &Vec<String> {
        self.os_packages.as_ref()
    }
}

impl Compose for JSGenerator {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        let project_path = self.options.get("path").unwrap();

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
                    "depends_on": self.images
                }
            }),
        )])
    }
}
