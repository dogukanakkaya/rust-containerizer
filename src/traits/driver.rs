use std::collections::HashMap;

pub trait Driver {
    fn generate(&self);
    fn find_images(&self) -> HashMap<String, String>;
}
