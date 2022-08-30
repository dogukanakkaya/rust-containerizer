pub mod elasticsearch;
pub mod mongo;
pub mod redis;

use self::{elasticsearch::Elasticsearch, mongo::Mongo, redis::Redis};
use crate::context::Image as ImageTrait;
use std::str::FromStr;

pub enum Image {
    Redis,
    Mongo,
    Elasticsearch,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "redis" => Ok(Self::Redis),
            "mongo" => Ok(Self::Mongo),
            "elasticsearch" => Ok(Self::Elasticsearch),
            _ => Err(String::from(format!("Image {} is not implemented yet.", s))),
        }
    }
}

// impl From<Image> for Box<dyn ImageTrait> {
//     fn from(image: Image) -> Self {
//         match image {
//             Image::Redis => Box::new(Redis::new()),
//             Image::Mongo => Box::new(Mongo::new()),
//         }
//     }
// }

// @TODO: add versions later (packages for each language is not compatible with the image's versions, figure it out)
impl Image {
    pub fn to_image(&self) -> Box<dyn ImageTrait> {
        match self {
            Self::Redis => Box::new(Redis::new()),
            Self::Mongo => Box::new(Mongo::new()),
            Self::Elasticsearch => Box::new(Elasticsearch::new()),
        }
    }

    pub fn filter_implemented_images(images: &Vec<String>) -> Vec<&String> {
        let mut implemented_images = vec![];

        for image in images {
            if let Ok(_) = image.parse::<Self>() {
                implemented_images.push(image);
            }
        }

        implemented_images
    }
}
