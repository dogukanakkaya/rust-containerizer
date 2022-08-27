use std::env;
use std::collections::HashMap;
use drivers::decider::Decider;

mod drivers;
// mod images;
mod traits;
mod os;

fn main() {
    let mut driver_options = HashMap::new();

    for arg in env::args() {
        if arg.starts_with("--") {
            match arg.find('=') {
                Some(i) => driver_options.insert(arg[2..i].to_string(), arg[i + 1..].to_string()),
                _ => None
            };
        }
    }

    let decider = Decider::from(driver_options);
    decider.decide();
}
