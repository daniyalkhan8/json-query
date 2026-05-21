use serde_json::Value;
use std::collections::HashMap;
use std::io::Error;
use std::{env, fs, process};

#[derive(Debug)]
struct Config {
    file_name: String,
    query: Vec<String>,
    compare_value: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, String> {
        let mut file_name = String::new();
        let mut query = String::new();
        let mut compare_value = String::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-f" => {i += 1; file_name = args[i].clone();}
                "-q" => {i += 1; query = args[i].clone();}
                "-v" => {i += 1; compare_value = args[i].clone();}
                _ => return Err(format!("Unknown flag found: {}", args[i].as_str()))
            }
            i += 1;
        }

        let query = query
            .split(".")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Config { file_name, query, compare_value })
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
    let mut cli_args = env::args();
    cli_args.next();

    let config = Config::new(cli_args.collect()).unwrap_or_else(|error| {
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
