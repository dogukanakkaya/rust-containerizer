mod elasticsearch;
mod mongo;
mod mysql;
mod postgresql;
mod redis;

use self::{
    elasticsearch::Elasticsearch, mongo::Mongo, mysql::MySQL, postgresql::PostgreSQL, redis::Redis,
};
use crate::context::Image as ImageTrait;
use std::str::FromStr;

pub enum Image {
    Redis,
    Mongo,
    Elasticsearch,
    MySQL,
    PostgreSQL,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "redis" => Ok(Self::Redis),
            "mongo" => Ok(Self::Mongo),
            "elasticsearch" => Ok(Self::Elasticsearch),
            "mysql" => Ok(Self::MySQL),
            "postgresql" => Ok(Self::PostgreSQL),
            _ => Err(String::from(format!("Image {} is not implemented yet.", s))),
        }
    }
}

impl Image {
    pub fn to_image(&self) -> Box<dyn ImageTrait> {
        match self {
            Self::Redis => Box::new(Redis::new()),
            Self::Mongo => Box::new(Mongo::new()),
            Self::Elasticsearch => Box::new(Elasticsearch::new()),
            Self::MySQL => Box::new(MySQL::new()),
            Self::PostgreSQL => Box::new(PostgreSQL::new()),
        }
    }
}
