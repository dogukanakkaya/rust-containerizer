pub mod js;
pub mod php;

use std::collections::HashMap;
use std::str::FromStr;

pub enum Driver {
    PHP,
    JS,
}

impl FromStr for Driver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "php" => Ok(Self::PHP),
            "js" => Ok(Self::JS),
            _ => Err(String::from(format!(
                "Driver {} is not implemented yet.",
                s
            ))),
        }
    }
}

pub trait DriverGenerator {
    fn generate(&self);
    fn add_to_ignore(&self, ignore: &mut String);
    fn find_images(&self) -> HashMap<String, String>;
}
