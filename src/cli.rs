use std::collections::HashMap;

static VALID_OPTIONS: [&str; 1] = ["--ignore-case"];

pub struct Cli {
    pub options: HashMap<String, String>,
    pub positional_arguments: Vec<String>,
}

impl Cli {
    pub fn parse<T>(args: T, limit: usize) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut options = HashMap::new();
        let mut positional_arguments = Vec::new();

        for arg in args.skip(1) {
            if arg.starts_with("--") {
                let mut parts = arg.splitn(2, '=');
                let key = parts.next().ok_or("Option malformed.")?;

                if VALID_OPTIONS.contains(&key) {
                    let value = &parts.next().ok_or("Option malformed.")?;
                    options.insert(
                        key.get(2..).ok_or("Option malformed")?.to_string(),
                        (*value).to_string(),
                    );
                } else {
                    return Err("Invalid option.");
                }
            } else {
                positional_arguments.push(arg);
            }
        }

        if positional_arguments.len() > limit {
            return Err("Too many arguments.");
        }

        Ok(Self {
            options,
            positional_arguments,
        })
    }
}
