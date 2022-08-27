use std::fs;

use serde_json::Value;

pub struct Package {
    filepath: String,
    data: Value
}

impl Package {
    pub fn new(filepath: String) -> Result<Self, String> {
        match fs::read_to_string(&filepath) {
            Ok(d) => {
                let data = serde_json::from_str(&d).expect(&format!("{} cannot be parsed to json.", filepath));
                Ok(Self { filepath, data })
            },
            Err(e) => Err(format!("Error while reading package.json file: {}", e))
        }
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn find_node_version(&self) -> String {
        self.data["engines"]["node"]
            .as_str()
            .unwrap_or("16")
            .chars()
            .filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x))
            .collect::<String>()
    }
}