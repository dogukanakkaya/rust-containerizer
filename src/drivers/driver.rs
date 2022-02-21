use std::str::FromStr;

pub enum Driver {
    PHP,
    NodeJS
}

impl FromStr for Driver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "php" => Ok(Self::PHP),
            "nodejs" => Ok(Self::NodeJS),
            _ => Err(String::from(format!("Driver {} is not implemented yet.", s)))
        }
    }
}