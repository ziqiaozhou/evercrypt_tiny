use config::Configuration;
mod bindgen;
mod config;
mod files;
mod make;

use make::Make;

fn main() {
    // Build the library
    println!("Building library...");
    let config = Configuration::new();
    let make = Make::new(config);
    make.build();

    // Generate bindings
    println!("Generating bindings...");
    bindgen::generate();
}
