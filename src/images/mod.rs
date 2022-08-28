pub mod elasticsearch;
pub mod mongodb;
pub mod redis;

use self::{elasticsearch::Elasticsearch, mongodb::MongoDB, redis::Redis};
use crate::context::Image as ImageTrait;
use std::str::FromStr;

pub enum Image {
    Redis,
    MongoDB,
    Elasticsearch,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "redis" => Ok(Self::Redis),
            "mongodb" => Ok(Self::MongoDB),
            "elasticsearch" => Ok(Self::Elasticsearch),
            _ => Err(String::from(format!("Image {} is not implemented yet.", s))),
        }
    }
}

// impl From<Image> for Box<dyn ImageTrait> {
//     fn from(image: Image) -> Self {
//         match image {
//             Image::Redis => Box::new(Redis::new()),
//             Image::MongoDB => Box::new(MongoDB::new()),
//         }
//     }
// }

impl Image {
    pub fn to_image(&self) -> Box<dyn ImageTrait> {
        match self {
            Self::Redis => Box::new(Redis::new()),
            Self::MongoDB => Box::new(MongoDB::new()),
            Self::Elasticsearch => Box::new(Elasticsearch::new()),
        }
    }
}
