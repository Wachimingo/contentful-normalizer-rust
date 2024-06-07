#![allow(dead_code, unused_variables, unused_mut)]
use structs::{includes_structs::Data, result_structs::{ParsedFieldsResult, ParsedIncludesEntryResult}, IncludesEntry};

use crate::string_helpers::to_camel_case;
use std::collections::HashMap;
pub mod structs;
use self::structs::{
    common_structs::ContentfulEntity,
    items_structs::Item,
    result_structs::{ParsedIncludesAssetEntry, ParsedIncludesEntry},
    ContentfulIncludes, ItemEntry,
};

pub fn normalize_labels(
    labels: Vec<ContentfulEntity>,
    includes: ContentfulIncludes,
) -> HashMap<String, String> {
    let mut record: HashMap<String, String> = HashMap::new();
    for label in labels {
        for entry in &includes.entries {
            if label.sys.id == entry.sys.id {
                if let Some(ref text) = entry.fields.text {
                    record.insert(to_camel_case(&entry.fields.slug), text.clone());
                }
            }
        }
    }
    return record;
}

pub fn normalize_configs(
    configs: Vec<ContentfulEntity>,
    includes: ContentfulIncludes,
) -> HashMap<String, Data> {
    let mut record: HashMap<String, Data> = HashMap::new();
    for config in configs {
        for entry in &includes.entries {
            if config.sys.id == entry.sys.id {
                // This way allows for coping the struct values, compared to match statement where data couldn't be moved
                if let Some(ref data) = entry.fields.data {
                    record.insert(to_camel_case(&entry.fields.slug), data.clone());
                }
            }
        }
    }
    return record;
}

pub fn process_item(
    item: Option<Item>,
    key: &str,
    includes: &ContentfulIncludes,
) -> Option<Vec<ParsedIncludesEntry>> {
    let mut items_collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    match item {
        Some(value) => match value {
            Item::Single(item) => {
                find_and_insert(
                    &item.sys.link_type,
                    &item.sys.id,
                    &key,
                    &includes,
                    &mut items_collector,
                );
            }
            Item::Multiple(items) => {
                for item in items {
                    find_and_insert(
                        &item.sys.link_type,
                        &item.sys.id,
                        &key,
                        &includes,
                        &mut items_collector,
                    );
                }
            }
        },
        None => {}
    };
    let item_arr = items_collector
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<ParsedIncludesEntry>>();
    if item_arr.is_empty() {
        None
    } else {
        Some(item_arr)
    }
}

pub fn find_data(
    includes_entry: &IncludesEntry,
    key: &str,
    includes: &ContentfulIncludes,
) -> ParsedIncludesEntryResult {
    let parsed_result = ParsedIncludesEntryResult {
        slug: includes_entry.fields.slug.clone(),
        title: includes_entry.fields.title.clone(),
        text: includes_entry.fields.text.clone(),
        link: includes_entry.fields.link.clone(),
        data: includes_entry.fields.data.clone(),
        fallback_image: includes_entry.fields.fallback_image.clone(),
        common_terms_and_conditions_items: includes_entry
            .fields
            .common_terms_and_conditions_items
            .clone(),
        confirmation_text: includes_entry.fields.confirmation_text.clone(),
        error_text: includes_entry.fields.error_text.clone(),
        confirm_button_text: includes_entry.fields.confirm_button_text.clone(),
        file: None,
        components: process_item(includes_entry.fields.components.clone(), key, includes),
        labels: process_item(includes_entry.fields.labels.clone(), key, includes),
        configs: process_item(includes_entry.fields.configs.clone(), key, includes),
        images: process_item(includes_entry.fields.images.clone(), key, includes),
    };
    parsed_result
}

pub fn find_and_insert(
    link_type: &str,
    id: &str,
    key: &str,
    includes: &ContentfulIncludes,
    collector: &mut HashMap<String, Vec<ParsedIncludesEntry>>,
) -> HashMap<String, Vec<ParsedIncludesEntry>> {
    if link_type == "Asset" {
        for asset in &includes.assets {
            if asset.sys.id == id {
                let parse_asset = ParsedIncludesAssetEntry {
                    slug: asset.fields.slug.clone(),
                    title: asset.fields.title.clone(),
                    text: asset.fields.text.clone(),
                    link: asset.fields.link.clone(),
                    data: asset.fields.data.clone(),
                    common_terms_and_conditions_items: asset
                        .fields
                        .common_terms_and_conditions_items
                        .clone(),
                    confirmation_text: asset.fields.confirmation_text.clone(),
                    error_text: asset.fields.error_text.clone(),
                    confirm_button_text: asset.fields.confirm_button_text.clone(),
                    url: asset.fields.file.as_ref().and_then(|file| file.url.clone()),
                };
                match collector.get_mut(key) {
                    Some(arr) => {
                        arr.push(ParsedIncludesEntry::Asset(parse_asset));
                    }
                    None => {
                        collector.insert(
                            key.to_string(),
                            vec![ParsedIncludesEntry::Asset(parse_asset)],
                        );
                    }
                };
            }
        }
    } else {
        for includes_entry in &includes.entries {
            if includes_entry.sys.id == id {
                match collector.get_mut(key) {
                    Some(arr) => {
                        let record = find_data(includes_entry, key, includes);
                        arr.push(ParsedIncludesEntry::Entry(record));
                    }
                    None => {
                        let record = find_data(includes_entry, key, includes);
                        collector.insert(
                            key.to_string(),
                            vec![ParsedIncludesEntry::Entry(record)],
                        );
                    }
                };
            }
        }
    }
    collector.clone()
}

pub fn parse_fields(
    entry: ItemEntry,
    includes: &ContentfulIncludes,
) -> ParsedFieldsResult {
    ParsedFieldsResult {                    
        title: entry.fields.title,
        slug: Some(entry.fields.slug),
        components: process_item(entry.fields.components.clone(), "components", includes),
        labels: process_item(entry.fields.labels.clone(), "labels", includes),
        configs: process_item(entry.fields.configs.clone(), "configs", includes),
    }
}

#[cfg(test)]
mod tests;
