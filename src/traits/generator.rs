pub trait Generator {
    fn generate(&self);
    fn find_images(&self) -> Vec<String>;
}
