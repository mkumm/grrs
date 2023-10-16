#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

struct CustomerError(String);

fn main() -> Result<(), CustomError> {

    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path).expect("could not read file");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.info()); }
    };
    println!("File content: {}", content);


    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
