#![allow(dead_code, unused_variables, unused_mut)]
use structs::{
    common_structs::ChildSys,
    includes_structs::Data,
    result_structs::{NormalizeResponseResult, ParsedIncludesEntryResult},
    ContentfulResponse, IncludesEntry,
};

use crate::string_helpers::to_camel_case;
use std::collections::HashMap;
pub mod structs;
use self::structs::{
    result_structs::{ParsedIncludesAssetEntry, ParsedIncludesEntry},
    ContentfulIncludes,
};

pub fn process_items_arr<'a>(
    items: Option<Vec<ChildSys>>,
    key: &str,
    includes: &ContentfulIncludes<'a>,
) -> Option<Vec<ParsedIncludesEntry<'a>>> {
    let mut items_collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    match items {
        Some(items) => {
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
        None => {}
    };
    let item_arr = items_collector
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<ParsedIncludesEntry<'a>>>();
    if item_arr.is_empty() {
        None
    } else {
        Some(item_arr)
    }
}

pub fn process_items<'a>(
    item: Option<ChildSys>,
    key: &str,
    includes: &ContentfulIncludes<'a>,
) -> Option<Vec<ParsedIncludesEntry<'a>>> {
    let mut items_collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    match item {
        Some(item) => {
            find_and_insert(
                &item.sys.link_type,
                &item.sys.id,
                &key,
                includes,
                &mut items_collector,
            );
        }
        None => {}
    };
    let item_arr = items_collector
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<ParsedIncludesEntry<'a>>>();
    if item_arr.is_empty() {
        None
    } else {
        Some(item_arr)
    }
}

pub fn find_data<'a>(
    includes_entry: &IncludesEntry<'a>,
    key: &str,
    includes: &ContentfulIncludes<'a>,
) -> ParsedIncludesEntryResult<'a> {
    let parsed_result = ParsedIncludesEntryResult {
        slug: includes_entry.fields.slug,
        title: includes_entry.fields.title,
        entry_title: includes_entry.fields.entry_title,
        text: includes_entry.fields.text,
        link: includes_entry.fields.link,
        data: includes_entry.fields.data.clone(),
        fallback_image: process_items(includes_entry.fields.fallback_image.clone(), key, includes),
        common_terms_and_conditions_items: process_items_arr(
            includes_entry
                .fields
                .common_terms_and_conditions_items
                .clone(),
            key,
            includes,
        ),
        confirmation_text: includes_entry.fields.confirmation_text,
        error_text: includes_entry.fields.error_text,
        confirm_button_text: includes_entry.fields.confirm_button_text,
        file: None,
        components: process_items_arr(includes_entry.fields.components.clone(), key, includes),
        labels: process_items_arr(includes_entry.fields.labels.clone(), key, includes),
        configs: process_items_arr(includes_entry.fields.configs.clone(), key, includes),
        images: process_items_arr(includes_entry.fields.images.clone(), key, includes),
    };
    parsed_result
}

pub fn find_and_insert<'a>(
    link_type: &str,
    id: &str,
    key: &str,
    includes: &ContentfulIncludes<'a>,
    collector: &mut HashMap<String, Vec<ParsedIncludesEntry<'a>>>,
) -> HashMap<String, Vec<ParsedIncludesEntry<'a>>> {
    if link_type == "Asset" {
        for asset in &includes.assets {
            if asset.sys.id == id {
                let parse_asset = ParsedIncludesAssetEntry {
                    slug: asset.fields.slug,
                    title: asset.fields.title,
                    text: asset.fields.text,
                    link: asset.fields.link,
                    data: asset.fields.data.clone(),
                    common_terms_and_conditions_items: asset
                        .fields
                        .common_terms_and_conditions_items
                        .clone(),
                    confirmation_text: asset.fields.confirmation_text,
                    error_text: asset.fields.error_text,
                    confirm_button_text: asset.fields.confirm_button_text,
                    url: asset.fields.file.as_ref().and_then(|file| file.url.clone()),
                    file: asset.fields.file.clone(),
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
                        let record = find_data(includes_entry, key, &includes);
                        arr.push(ParsedIncludesEntry::Entry(record));
                    }
                    None => {
                        let record = find_data(includes_entry, key, &includes);
                        collector.insert(key.to_string(), vec![ParsedIncludesEntry::Entry(record)]);
                    }
                };
            }
        }
    }
    collector.clone()
}

pub fn normalize_labels<'a>(
    labels: Vec<ChildSys>,
    includes: &ContentfulIncludes<'a>,
) -> HashMap<String, &'a str> {
    let mut record: HashMap<String, &'a str> = HashMap::new();
    for label in labels {
        for entry in &includes.entries {
            if label.sys.id == entry.sys.id {
                if let Some(ref text) = entry.fields.text {
                    record.insert(to_camel_case(&entry.fields.slug), text);
                }
            }
        }
    }
    return record;
}

pub fn normalize_configs(
    configs: Vec<ChildSys>,
    includes: &ContentfulIncludes,
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

pub fn normalize_components<'a>(
    components: Option<Vec<ChildSys>>,
    includes: &ContentfulIncludes<'a>,
) -> HashMap<String, Option<Vec<ParsedIncludesEntry<'a>>>> {
    let mut record: HashMap<String, Option<Vec<ParsedIncludesEntry>>> = HashMap::new();
    match components {
        Some(components) => {
            for component in components {
                let entry = includes
                    .entries
                    .clone()
                    .into_iter()
                    .find(|entry| entry.sys.id == component.sys.id)
                    .unwrap();
                record.insert(
                    to_camel_case(&entry.fields.slug),
                    process_items(Some(component), "components", &includes),
                );
            }
        }
        None => {}
    }
    record
}

pub fn normalize_response(response: ContentfulResponse, slug: String) -> NormalizeResponseResult {
    let main_page_entry = response
        .items
        .into_iter()
        .find(|item| item.fields.slug == slug)
        .unwrap();
    NormalizeResponseResult {
        slug: Some(main_page_entry.fields.slug),
        labels: Some(normalize_labels(
            main_page_entry.fields.labels.unwrap(),
            &response.includes,
        )),
        configs: Some(normalize_configs(
            main_page_entry.fields.configs.unwrap(),
            &response.includes,
        )),
        components: Some(normalize_components(
            main_page_entry.fields.components,
            &response.includes,
        )),
    }
}

#[cfg(test)]
mod tests;
