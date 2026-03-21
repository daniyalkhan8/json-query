use std::{env, process};
use std::fs::File;
use std::io::{BufReader, Error};
use serde_json::Value;

#[derive(Debug)]
struct Config {
    query: String,
    file_name: String
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("required query not found")
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("required file_name not found")
        };

        Ok(Config { query, file_name })
    }
}

fn read_json(file_name: &str) -> Result<Value, Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let value: Value = serde_json::from_reader(reader)?;
    Ok(value)
}

fn main() {
    let cli_args = env::args();
    let config = Config::new(cli_args).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{config:?}");

    let json_value = read_json(&config.file_name).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{json_value}");
}
