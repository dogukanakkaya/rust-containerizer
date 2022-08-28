mod compose;
mod context;
mod drivers;
mod images;
mod os;
mod traits;

use context::Context;
use std::collections::HashMap;
use std::env;

fn main() {
    let mut driver_options = HashMap::new();

    for arg in env::args() {
        if arg.starts_with("--") {
            match arg.find('=') {
                Some(i) => driver_options.insert(arg[2..i].to_string(), arg[i + 1..].to_string()),
                None => driver_options.insert(arg[2..].to_string(), "true".to_string()),
            };
        }
    }

    let context = Context::from(driver_options);
    context.exec();
}
