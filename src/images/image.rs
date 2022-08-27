use std::str::FromStr;

pub enum Image {
    Redis,
    MongoDB,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "redis" => Ok(Self::Redis),
            "mongodb" => Ok(Self::MongoDB),
            _ => Err(String::from(format!("Image {} is not implemented yet.", s))),
        }
    }
}
