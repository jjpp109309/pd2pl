use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub output_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path :(")
        };

        let output_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get output path :(")
        };

        Ok(Config { file_path, output_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let alias = get_alias(&contents);

    Ok(())
}

pub fn get_alias(contents: &str) -> &str {
// pub fn get_alias<'a>(contents: &'a str) -> &str {
    let pattern = String::from("import pandas as");

    let pattern_line = contents
        .lines()
        .filter(|line| line.starts_with(&pattern))
        .next();

    let (_, alias) = pattern_line
        .unwrap()
        .rsplit_once(" ")
        .unwrap();

    alias
}
