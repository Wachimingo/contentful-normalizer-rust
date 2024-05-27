use std::fs;

use clap::Parser;
use rust_contentful_normalizer::normalize_helpers::structs::ContentfulResponse;

#[derive(Parser, Default)]
struct CLI {
    #[clap(short = 'f', long)]
    file_path: std::path::PathBuf,    
}

fn main() {
    let args = CLI::parse();
    let file = fs::read_to_string(args.file_path).expect("Error reading file");
    
    let parsed_file: ContentfulResponse = serde_json::from_str(&file).expect("coud not parse");

    println!("{:?}", parsed_file);
}
