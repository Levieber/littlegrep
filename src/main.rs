use std::{env, process};

use littlegrep::config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem when parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = littlegrep::run(config) {
        eprintln!("Application error: {error}");
        process::exit(1)
    }
}
