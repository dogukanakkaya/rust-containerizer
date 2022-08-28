use std::collections::HashMap;

pub trait Generator {
    fn generate(&self);
    fn find_images(&self) -> HashMap<String, String>;
}
