use config::Configuration;
mod config;
mod files;
mod make;

use make::Make;

fn main() {
    let config = Configuration::new();
    let make = Make::new(config);
    make.build();
}
