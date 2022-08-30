use serde_json::Value;
use std::{collections::HashMap, fs};

pub struct Composer {
    data: Value,
}

impl Composer {
    pub fn new(filepath: String) -> Result<Self, String> {
        match fs::read_to_string(&filepath) {
            Ok(d) => {
                let data = serde_json::from_str(&d)
                    .expect(&format!("{} cannot be parsed to json.", filepath));
                Ok(Self { data })
            }
            Err(e) => Err(format!("Error while reading composer.json file: {}", e)),
        }
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn find_php_version(&self) -> String {
        self.data["require"]["php"]
            .as_str()
            .unwrap_or("latest")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>()
    }

    pub fn all_dependencies(&self) -> HashMap<&String, &serde_json::Value> {
        self.data()["require"]
            .as_object()
            .unwrap()
            .into_iter()
            .chain(self.data()["require-dev"].as_object().unwrap())
            .collect::<HashMap<&String, &serde_json::Value>>()
    }
}
