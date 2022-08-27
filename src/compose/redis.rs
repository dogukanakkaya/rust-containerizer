use super::composer::Composer;
use crate::drivers::js::package::Package;
use crate::traits::generator::Generator;
use std::{fs::File, io::Write};

pub struct Redis {}

impl Generator for Redis {
    fn generate(project_path: &String) {}
}
