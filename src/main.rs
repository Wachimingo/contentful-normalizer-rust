use std::{ffi::OsStr, fs, path::PathBuf};
use log::{info, debug};
use clap::Parser;
use rust_contentful_normalizer::normalize_helpers::{normalize_response, structs::ContentfulResponse};

#[derive(Parser, Default)]
struct CLI {
    #[clap(short = 'f', long)]
    file_path: std::path::PathBuf,
}

fn main() {
    env_logger::init();

    info!("Starting up");
    let args = CLI::parse();
    debug!("Parsed CLI arguments");
    
    let file = fs::read_to_string(&args.file_path).expect("Error reading file");
    debug!("Found and read file");

    let parsed_file: ContentfulResponse = serde_json::from_str(&file).expect("coud not parse");
    debug!("Parsed json file contents to structs");

    let res = normalize_response(parsed_file, "product-subscription-overview-main-page".to_string());
    info!("Normalized response");

    let mut path = PathBuf::from(args.file_path);
    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");
    path.set_file_name(format!("{}_parsed.{}", file_stem, extension));

    fs::write(path, serde_json::to_string(&res).expect("couldn't parsed to json")).expect("couldn't write to file");
}
