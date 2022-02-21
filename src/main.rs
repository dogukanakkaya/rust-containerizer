use std::env;
use std::collections::HashMap;

mod drivers;

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
    println!("driver = {:?}", driver_options);
}
