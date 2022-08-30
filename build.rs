use std::{process::Command, thread};

/// The path to the current vendored distribution
const DIST: &str = "vendored/v0.4.5-dist";

/// Executes a command
#[cfg(target_family = "unix")]
fn shell_exec(pwd: &str, command: &str) {
    // Execute command
    let result = match Command::new("sh").arg("-c").arg(command).current_dir(pwd).status() {
        Ok(result) => result,
        Err(e) => panic!("Failed to execute command: {command} ({e})"),
    };

    // Check result
    if !result.success() {
        panic!("Command failed: {result}")
    }
}
/// Executes a command
#[cfg(target_family = "windows")]
fn shell_exec(pwd: &str, command: &str) {
    // Execute command
    let result = match Command::new("bash.exe").arg("-c").arg(command).current_dir(pwd).status() {
        Ok(result) => result,
        Err(e) => panic!("Failed to execute command: {command} ({e})"),
    };

    // Check result
    if !result.success() {
        panic!("Command failed: {result}")
    }
}
/// Executes a command
#[cfg(not(any(target_family = "unix", target_family = "windows")))]
fn shell_exec(_pwd: &str, _command: &str) {
    panic!("Current target platform is not supported")
}

fn main() {
    // Determine parallelism
    let threads = thread::available_parallelism().map(usize::from).unwrap_or(1);
    let make = format!("make -j{threads}");

    // Build library
    let dir = format!("{DIST}/c89-compatible");
    shell_exec(&dir, "./configure --disable-ocaml");
    shell_exec(&dir, &make);

    // Link library
    println!("cargo:rustc-link-search=native={dir}");
    println!("cargo:rustc-link-lib=static=evercrypt");
}
