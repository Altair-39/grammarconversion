mod converter;
use crate::converter::converter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <grammar_file_path>");
        std::process::exit(1);
    }

    let grammar_file = &args[1];

    if let Err(e) = converter(grammar_file) {
        eprintln!("Error: {}", e);
    }
}
