use std::env;

use crate::cli::Arguments;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<T>(args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let cli = Arguments::new(args, 2)?;

        let query = cli
            .positional_arguments
            .first()
            .ok_or("Query not provided")?
            .to_string();
        let file_path = cli
            .positional_arguments
            .get(1)
            .ok_or("File path not provided")?
            .to_string();
        let ignore_case = env::var("IGNORE_CASE").map_or(false, |v| {
            v == "true"
                || cli
                    .options
                    .get("ignore-case")
                    .map_or(false, |v| v == "true")
        });

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
