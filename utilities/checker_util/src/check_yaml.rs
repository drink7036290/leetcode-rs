use serde_yaml::{Error, Value};
use std::fs;

pub fn check_yaml() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("No files to check.");
        std::process::exit(1);
    }

    let mut has_error = false;
    for file in args {
        match fs::read_to_string(&file) {
            Ok(content) => {
                let parse_result: Result<Value, Error> = serde_yaml::from_str(&content);
                if parse_result.is_err() {
                    eprintln!(
                        "YAML parse error in {}:\n{}",
                        file,
                        parse_result.unwrap_err()
                    );
                    has_error = true;
                }
            }
            Err(err) => {
                eprintln!("Could not read {}: {}", file, err);
                has_error = true;
            }
        }
    }
    if has_error {
        std::process::exit(1);
    }
}
