use std::{env, process};

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

fn main() {
    let cli_args = env::args();
    let config = Config::new(cli_args).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{config:?}")
}
