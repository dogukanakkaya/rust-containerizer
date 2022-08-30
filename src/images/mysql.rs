use crate::compose::Compose;
use serde_json::json;
use std::collections::HashMap;

pub struct MySQL {}

impl MySQL {
    pub fn new() -> Self {
        Self {}
    }
}

impl Compose for MySQL {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value> {
        HashMap::from([
            (
                "services",
                json!({
                    "mysql": {
                        "image": "mysql",
                        "ports": [
                            "3306:3306"
                        ],
                        "volumes": [
                            "mysql_data:/var/lib/mysql",
                        ],
                        "environment": [
                            "MYSQL_DATABASE=${DATABASE_NAME}",
                            "MYSQL_ROOT_PASSWORD=${DATABASE_ROOT_PASSWORD}"
                        ],
                        "command": "--default-authentication-plugin=mysql_native_password // do not use in production",
                        "restart": "always"
                    }
                }),
            ),
            (
                "volumes",
                json!({
                    "mysql_data": {}
                }),
            ),
        ])
    }
}
