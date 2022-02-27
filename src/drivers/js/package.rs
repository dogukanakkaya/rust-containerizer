use std::fs;

use json::JsonValue;

pub struct Package {
    filepath: String,
    data: JsonValue
}

impl Package {
    pub fn new(filepath: String) -> Result<Self, String> {
        match fs::read_to_string(&filepath) {
            Ok(d) => {
                let data = json::parse(&d).expect(&format!("{} cannot be parsed to json.", filepath));
                Ok(Self {filepath, data })
            },
            Err(e) => Err(format!("Error while reading package.json file: {}", e))
        }
    }

    pub fn data(&self) -> &JsonValue {
        &self.data
    }
}