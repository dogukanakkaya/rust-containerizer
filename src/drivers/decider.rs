use std::collections::HashMap;
use super::driver::Driver;

#[derive(Debug)]
pub struct Decider {
    driver_options: HashMap<String, String>
}

impl From<HashMap<String, String>> for Decider {
    fn from(driver_options: HashMap<String, String>) -> Self {
        Self {
            driver_options
        }
    }
}

impl Decider {
    pub fn decide(&self) {
        let driver: Driver = self.driver_options.get("driver").expect("Driver is missing from options. Did you miss --driver option?").parse().unwrap();

        match driver {
            Driver::PHP => unimplemented!(),
            Driver::NodeJS => unimplemented!(),
            _ => unimplemented!()
        }
    }
}