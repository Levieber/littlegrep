use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line)
        }
    }

    result
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let mut query: Option<String> = None;
        let mut file_path: Option<String> = None;
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                _ if arg.starts_with("--ignore-case") => {
                    let value = &arg["--ignore-case=".len()..];
                    match value {
                        "true" => ignore_case = true,
                        "false" => ignore_case = false,
                        _ => {
                            return Err(
                                "Invalid value for --ignore-case. Use \"true\" or \"false\".",
                            )
                        }
                    }
                }
                _ if query.is_none() => query = Some(arg.clone()),
                _ if file_path.is_none() => file_path = Some(arg.clone()),
                _ => return Err("Too many arguments provided."),
            }
        }

        let query = query.ok_or("Query not provided")?;
        let file_path = file_path.ok_or("File path not provided")?;

        Ok(Config {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
