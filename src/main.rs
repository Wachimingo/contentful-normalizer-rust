use clap::Parser;
use rust_contentful_normalizer::normalize_helpers::{
    normalize_components, normalize_configs, normalize_labels, structs::{result_structs::NormalizeResponseResult, ContentfulResponse}
};
use std::{ffi::OsStr, fs, path::PathBuf};

#[derive(Parser, Default)]
struct CLI {
    #[clap(short = 'f', long)]
    file_path: std::path::PathBuf,
    #[clap(short = 's', long)]
    slug: String,
}

fn main() {
    let args = CLI::parse();

    let file = fs::read_to_string(&args.file_path).expect("Error reading file");

    let parsed_file: ContentfulResponse = serde_json::from_str(&file).expect("coud not parse");

    let main_page_entry = parsed_file
        .items
        .into_iter()
        .find(|item| item.fields.slug == args.slug)
        .unwrap();
    let res = NormalizeResponseResult {
        slug: Some(main_page_entry.fields.slug),
        labels: Some(normalize_labels(
            main_page_entry.fields.labels.unwrap(),
            &parsed_file.includes,
        )),
        configs: Some(normalize_configs(
            main_page_entry.fields.configs.unwrap(),
            &parsed_file.includes,
        )),
        components: Some(normalize_components(
            main_page_entry.fields.components.unwrap(),
            &parsed_file.includes,
        )),
    };

    let mut path = PathBuf::from(args.file_path);
    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");
    path.set_file_name(format!("{}_parsed.{}", file_stem, extension));

    fs::write(
        path,
        serde_json::to_string(&res).expect("couldn't parsed to json"),
    )
    .expect("couldn't write to file");
}
