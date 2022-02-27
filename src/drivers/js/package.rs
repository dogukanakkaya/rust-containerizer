use std::fs;

use json::JsonValue;

pub struct Package {
    filepath: String,
    data: JsonValue
}

impl Package {
    pub fn new(filepath: String) -> Result<Self, String> {
        let data = json::parse(&fs::read_to_string(&filepath).unwrap());

        match data {
            Ok(d) => Ok(Self {filepath, data: d}),
            Err(e) => Err(format!("Error while reading package.json file: {}", e))
        }
    }

    pub fn data(&self) -> &JsonValue {
        &self.data
    }
}