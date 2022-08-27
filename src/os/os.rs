use std::str::FromStr;

pub enum Os {
    Ubuntu,
    Alpine
}

impl FromStr for Os {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ubuntu" => Ok(Self::Ubuntu),
            "alpine" => Ok(Self::Alpine),
            _ => Err(String::from(format!("OS {} is not implemented yet.", s)))
        }
    }
}