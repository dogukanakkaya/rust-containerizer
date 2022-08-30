use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct PostgreSQL {}

impl PostgreSQL {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for PostgreSQL {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                    "postgresql": {
                        "image": "postgres",
                        "ports": [
                            "5432:5432"
                        ],
                        "volumes": [
                            "postgresql_data:/var/lib/postgresql/data",
                        ],
                        "environment": [
                            "POSTGRES_USER=postgres",
                            "POSTGRES_PASSWORD=${DATABASE_PASSWORD}",
                            "POSTGRES_HOST_AUTH_METHOD=trust // do not use in production"
                        ],
                        "restart": "always"
                    }
                }),
            ),
            (
                "volumes",
                json!({
                    "postgresql_data": {}
                }),
            ),
        ])
    }
}
