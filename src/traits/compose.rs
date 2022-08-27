use std::collections::HashMap;

pub trait Compose {
    fn find_compose_definition(&self) -> HashMap<&str, serde_json::Value>;

    fn add_to_compose(
        definition: HashMap<&str, serde_json::Value>,
        compose: &mut serde_json::Value,
    ) {
        for (key, value) in definition {
            for (k, v) in value.as_object().unwrap().to_owned() {
                compose[key].as_object_mut().unwrap().insert(k, v);
            }
        }
    }
}
