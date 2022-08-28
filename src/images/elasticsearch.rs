use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct Elasticsearch {}

impl Elasticsearch {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for Elasticsearch {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                        "elasticsearch": {
                            "image": "elasticsearch",
                            "volumes": [
                                "es_data:/usr/share/elasticsearch/data",
                            ],
                            "environment": [
                                "discovery.type=single-node"
                            ]
                        }
                }),
            ),
            (
                "volumes",
                json!({
                    "es_data": ""
                }),
            ),
        ])
    }
}
