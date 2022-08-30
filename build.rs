use std::{process::Command, thread};

/// The path to the current vendored distribution
const DIST: &str = "vendored/v0.4.5-dist";

/// Executes a command
fn exec<const LEN: usize>(cd: &str, command: &str, args: [&str; LEN]) {
    // Execute command
    let command = format!("{cd}/{command}");
    let result = match Command::new(&command).args(args).current_dir(cd).status() {
        Ok(result) => result,
        Err(e) => panic!("Failed to execute command: {command} {args:?} ({e})"),
    };

    // Check result
    if !result.success() {
        panic!("Command failed: {result}")
    }
}

fn main() {
    // Determine parallelism
    let threads = thread::available_parallelism().map(usize::from).unwrap_or(1);
    let parallelism = format!("-j{threads}");

    // Build library
    let dir = format!("{DIST}/c89-compatible");
    exec(&dir, "configure", ["--disable-ocaml"]);
    exec(&dir, "make", [&parallelism]);

    // Link library
    println!("cargo:rustc-link-search=native={dir}");
    println!("cargo:rustc-link-lib=static=evercrypt");
}
