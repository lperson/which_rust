use std::fs::File;
use std::path::Path;

use clap;
use dirs;

pub struct Config {
    pub prefix: String,
    pub suffix: String,
}

impl Config {
    pub fn from_command_line() -> Config {
        let matches = clap::App::new("which_rust")
            .version("0.1.0")
            .about("Which Rust is active in the current directory?")
            .arg(
                clap::Arg::with_name("prefix")
                    .short("p")
                    .long("prefix")
                    .takes_value(true)
                    .help("Text to display beforee the Rust version"),
            )
            .arg(
                clap::Arg::with_name("suffix")
                    .short("s")
                    .long("suffix")
                    .takes_value(true)
                    .help("Text to display after the Rust version"),
            )
            .get_matches();

        return Config {
            prefix: matches.value_of("prefix").unwrap_or("").to_string(),
            suffix: matches.value_of("suffix").unwrap_or("").to_string(),
        };
    }
}

pub fn is_cargo_toml_in_this_directory(path: &Path) -> bool {
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
