use std::env;

use crate::cli::Cli;

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
        let cli = Cli::parse(args, 2)?;

        let query = cli
            .positional_arguments
            .first()
            .ok_or("Query not provided.")?
            .to_string();
        let file_path = cli
            .positional_arguments
            .get(1)
            .ok_or("File Path not provided.")?
            .to_string();

        let ignore_case = env::var("IGNORE_CASE").map_or(false, |v| v == "true");
        let ignore_case = cli
            .options
            .get("ignore-case")
            .map_or(ignore_case, |v| v == "true");

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_successfully() {
        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=true".to_string(),
            ]
            .into_iter(),
        ).unwrap();

        assert_eq!(config.query, "to");
        assert_eq!(config.file_path, "poem.txt");
        assert_eq!(config.ignore_case, true);
    }

    #[test]
    fn query_not_provided() {
        let config = Config::build([String::new()].into_iter());

        assert_eq!(config.err().unwrap(), "Query not provided.");
    }

    #[test]
    fn file_path_not_provided() {
        let config = Config::build([String::new(), "to".to_string()].into_iter());

        assert_eq!(config.err().unwrap(), "File Path not provided.");
    }

    #[test]
    fn ignore_case_flag_true_and_env_var_not_specified() {
        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=true".to_string(),
            ]
            .into_iter(),
        );

        assert!(config.unwrap().ignore_case);
    }

    #[test]
    fn ignore_case_env_var_true_and_flag_not_specified() {
        env::set_var("IGNORE_CASE", "true");

        let config =
            Config::build([String::new(), "to".to_string(), "poem.txt".to_string()].into_iter());

        assert!(config.unwrap().ignore_case);
    }

    #[test]
    fn ignore_case_env_var_and_flag_true() {
        env::set_var("IGNORE_CASE", "true");

        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=true".to_string(),
            ]
            .into_iter(),
        );

        assert!(config.unwrap().ignore_case);
    }

    #[test]
    fn ignore_case_env_var_and_flag_false() {
        env::set_var("IGNORE_CASE", "false");

        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=false".to_string(),
            ]
            .into_iter(),
        );

        assert!(!config.unwrap().ignore_case);
    }

    #[test]
    fn ignore_case_env_var_false_and_flag_true() {
        env::set_var("IGNORE_CASE", "false");

        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=true".to_string(),
            ]
            .into_iter(),
        );

        assert!(config.unwrap().ignore_case);
    }

    #[test]
    fn ignore_case_env_var_true_and_flag_false() {
        env::set_var("IGNORE_CASE", "true");

        let config = Config::build(
            [
                String::new(),
                "to".to_string(),
                "poem.txt".to_string(),
                "--ignore-case=false".to_string(),
            ]
            .into_iter(),
        );

        assert!(!config.unwrap().ignore_case);
    }
}
