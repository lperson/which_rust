use std::env;
use std::process;
use std::process::Command;
use std::vec;

mod lib;
use lib::is_cargo_toml_in_this_directory;
use lib::Config;

fn main() {

    let config = Config::from_command_line();

    if let Ok(current_dir) = env::current_dir() {
        if !is_cargo_toml_in_this_directory(&current_dir.as_path()) {
            process::exit(1);
        }
    } else {
        process::exit(1);
    }

    let mut output: process::Output;
    if let Ok(output_result) = Command::new("rustc").arg("--version").output() {
        output = output_result;
    } else {
        process::exit(1);
    }

    if let Ok(full_version_string) = std::str::from_utf8(output.stdout.as_slice()) {
        let split_version: vec::Vec<&str> = full_version_string.split(' ').collect();
        if split_version.len() < 2 {
            process::exit(1);
        }
        println!("{prefix}{version}{suffix}", 
            prefix=config.prefix,
            version=split_version[1],
            suffix=config.suffix
        );
    } else {
        process::exit(1)
    }
}
