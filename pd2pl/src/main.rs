use std::env;
use std::process;
use pd2pl::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("file path {}", config.file_path);
    println!("output path {}", config.output_path);
}
