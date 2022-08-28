pub mod mongodb;
pub mod redis;

use std::collections::HashMap;

pub trait Compose {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value>;

    fn add_to_compose(&self, compose: &mut serde_json::Value) {
        let definition = self.find_compose_definition();

        for (key, value) in definition {
            for (k, v) in value.as_object().unwrap().to_owned() {
                compose[key].as_object_mut().unwrap().insert(k, v);
            }
        }
    }
}
