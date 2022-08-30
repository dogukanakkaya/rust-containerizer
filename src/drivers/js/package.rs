use serde_json::Value;
use std::{collections::HashMap, fs};

pub struct Package {
    data: Value,
}

impl Package {
    pub fn new(filepath: String) -> Result<Self, String> {
        match fs::read_to_string(&filepath) {
            Ok(d) => {
                let data = serde_json::from_str(&d)
                    .expect(&format!("{} cannot be parsed to json.", filepath));
                Ok(Self { data })
            }
            Err(e) => Err(format!("Error while reading package.json file: {}", e)),
        }
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn find_node_version(&self) -> String {
        self.data["engines"]["node"]
            .as_str()
            .unwrap_or("18")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>()
    }

    // merge dependencies and devDependencies to single map
    pub fn all_dependencies(&self) -> HashMap<&String, &serde_json::Value> {
        self.data["dependencies"]
            .as_object()
            .unwrap()
            .into_iter()
            .chain(self.data()["devDependencies"].as_object().unwrap())
            .collect::<HashMap<&String, &serde_json::Value>>()
    }
}
