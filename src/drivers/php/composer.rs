use std::fs;

use json::JsonValue;

pub struct Composer {
    filepath: String,
    data: JsonValue
}

impl Composer {
    pub fn new(filepath: String) -> Self {
        let data = json::parse(&fs::read_to_string(&filepath).unwrap()).expect(&format!("{} cannot be parsed to json.", filepath));

        Self {filepath, data}
    }

    pub fn data(&self) -> &JsonValue {
        &self.data
    }
}