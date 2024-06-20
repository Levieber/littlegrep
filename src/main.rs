use std::{env, process};

use littlegrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem when parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = littlegrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
