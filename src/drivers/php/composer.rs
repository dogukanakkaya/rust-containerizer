use std::fs;

use serde_json::Value;

pub struct Composer {
    filepath: String,
    data: Value,
}

impl Composer {
    pub fn new(filepath: String) -> Result<Self, String> {
        match fs::read_to_string(&filepath) {
            Ok(d) => {
                let data = serde_json::from_str(&d)
                    .expect(&format!("{} cannot be parsed to json.", filepath));
                Ok(Self { filepath, data })
            }
            Err(e) => Err(format!("Error while reading composer.json file: {}", e)),
        }
    }

    pub fn data(&self) -> &Value {
        &self.data
    }
}
