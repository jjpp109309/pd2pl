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