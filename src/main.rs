use std::fs;

use clap::Parser;
use rust_contentful_normalizer::normalize_helpers::{parse_fields, structs::ContentfulResponse};

#[derive(Parser, Default)]
struct CLI {
    #[clap(short = 'f', long)]
    file_path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();
    let file = fs::read_to_string(args.file_path).expect("Error reading file");
    
    let parsed_file: ContentfulResponse = serde_json::from_str(&file).expect("coud not parse");
    let res = parse_fields(parsed_file.items[0].clone(), &parsed_file.includes);
    println!("{:?}", serde_json::to_string(&res).expect("error"));
}
