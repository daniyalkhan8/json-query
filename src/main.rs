use serde_json::Value;
use std::collections::HashMap;
use std::io::Error;
use std::{env, fs, process};

#[derive(Debug)]
struct Config {
    query: Vec<String>,
    file_name: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("required query not found"),
        };

        let query: Vec<String> = query
            .split(".")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("required file_name not found"),
        };

        Ok(Config { query, file_name })
    }
}

#[derive(Debug)]
struct JsonObject {
    json_object: Vec<HashMap<String, Value>>,
}

impl JsonObject {
    fn new(file_name: &str) -> Result<JsonObject, Error> {
        let json = fs::read_to_string(file_name)?;
        let json_object: Vec<HashMap<String, Value>> = serde_json::from_str(&json)?;

        Ok(JsonObject { json_object })
    }

    fn query(&self, query: &Vec<String>) -> Vec<&Value> {
        let mut result: Vec<&Value> = Vec::new();

        for object in &self.json_object {
            let mut keys = query.iter();
            let mut current_value = keys.next().and_then(|k| object.get(k));

            for key in keys {
                current_value = current_value.and_then(|v| v.get(key));
            }

            if let Some(value) = current_value {
                result.push(value);
            }
        }

        result
    }
}

fn main() {
    let cli_args = env::args();
    let config = Config::new(cli_args).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    println!("{config:?}");

    let json_object = JsonObject::new(&config.file_name).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    let user_names = json_object.query(&config.query);
    println!("{user_names:?}");
}
