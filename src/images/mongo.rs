use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct Mongo {}

impl Mongo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for Mongo {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                    "mongo": {
                        "image": "mongo",
                        "ports": [
                            "27017:27017"
                        ],
                        "volumes": [
                            "mongo_data:/data/db",
                        ],
                        "environment": [
                            "MONGO_INITDB_ROOT_USERNAME=root",
                            "MONGO_INITDB_ROOT_PASSWORD=123456"
                        ]
                    }
                }),
            ),
            (
                "volumes",
                json!({
                    "mongo_data": {}
                }),
            ),
        ])
    }
}
