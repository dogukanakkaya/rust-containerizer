use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct Redis {}

impl Redis {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for Redis {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                    "redis": {
                        "image": "redis",
                        "ports": [
                            "6379:6379"
                        ],
                        "volumes": [
                            "redis_data:/data",
                        ]
                    }
                }),
            ),
            (
                "volumes",
                json!({
                    "redis_data": {}
                }),
            ),
        ])
    }
}
