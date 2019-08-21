use std::env;
use std::process;
use std::process::Command;
use std::vec;

use std::path::{Path, PathBuf};
use std::fs::File;

use dirs;

fn is_cargo_toml_in_this_directory(path: &Path) -> bool {
    if let Some(parent) = path.parent() {
        if let Some(home_dir) = dirs::home_dir() {
            if home_dir.as_path() == path {
                return false;
            }
        }

        let full_path = path.join("Cargo.toml");

        if File::open(full_path).is_ok() {
            return true;
        }

        return is_cargo_toml_in_this_directory(parent);
    } else {
        return false;
    }
}

fn main() {

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
        println!(" {}", split_version[1]);
    } else {
        process::exit(1)
    }
}
