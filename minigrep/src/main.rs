use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("Searching for {}", config.query);
    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
