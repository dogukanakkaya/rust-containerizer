use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct MongoDB {}

impl MongoDB {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for MongoDB {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                    "mongodb": {
                        "image": "mongodb",
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
                    "mongo_data": ""
                }),
            ),
        ])
    }
}
