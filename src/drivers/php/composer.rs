use std::fs;

use json::JsonValue;

#[derive(Debug)]
pub struct Composer {
    filepath: String,
    version: u8,
    data: JsonValue
}

impl Composer {
    pub fn new(filepath: String, version: u8) -> Self {
        let data = json::parse(&fs::read_to_string(&filepath).unwrap());

        match data {
            Ok(d) => Self {filepath, version, data: d},
            Err(e) => panic!("Error while reading composer.json file: {}", e)
        }
    }

    pub fn data(&self) -> &JsonValue {
        &self.data
    }
}