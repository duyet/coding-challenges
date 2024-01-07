use clap::Parser;
use std::path::PathBuf;

mod json_parser;

/// JSON Parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = std::fs::read_to_string(args.file).expect("Failed to read file");
    let json = json_parser::parse_json_file(&file).expect("Failed to parse JSON");

    println!("{:#?}", json_parser::serialize_jsonvalue(&json));
}
