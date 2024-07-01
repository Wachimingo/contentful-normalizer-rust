use structs::{
    common_structs::ChildSys,
    includes_structs::Data,
    result_structs::{NormalizeResponseResult, ParsedFieldsResult, ParsedIncludesEntryResult},
    ContentfulResponse, IncludesEntry, ItemEntry,
};

use crate::string_helpers::to_camel_case;
use std::{collections::HashMap, rc::Rc};
pub mod structs;
use self::structs::{
    items_structs::Item,
    result_structs::{ParsedIncludesAssetEntry, ParsedIncludesEntry},
    ContentfulIncludes,
};

pub fn process_item<'a>(
    item: Item<ChildSys<'a>>,
    key: &'a str,
    includes: Rc<ContentfulIncludes<'a>>,
) -> Vec<ParsedIncludesEntry<'a>> {
    let mut items_collector: HashMap<&'a str, Vec<ParsedIncludesEntry<'a>>> = HashMap::new();
    match item {
        Item::Single(item) => {
            let result =
                find_and_insert(item.sys.link_type, item.sys.id, key, includes);
            match result {
                Some(record) => {
                    match items_collector.get_mut(key) {
                        Some(arr) => {
                            arr.push(record);
                        }
                        None => {
                            items_collector.insert(key, vec![record]);
                        }
                    };
                }
                None => {}
            }
        }
        Item::Multiple(items) => {
            for item in items {
                let result =
                    find_and_insert(item.sys.link_type, item.sys.id, key, includes.clone());
                match result {
                    Some(record) => {
                        match items_collector.get_mut(key) {
                            Some(arr) => {
                                arr.push(record);
                            }
                            None => {
                                items_collector.insert(key, vec![record]);
                            }
                        };
                    }
                    None => {}
                }
            }
        }
    }
    let item_arr = items_collector
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<ParsedIncludesEntry>>();
    item_arr
}

pub fn parse_data<'a>(
    includes_entry: &'a IncludesEntry<'a>,
    key: &'a str,
    includes: Rc<ContentfulIncludes<'a>>,
) -> ParsedIncludesEntryResult<'a> {    
    ParsedIncludesEntryResult {
        slug: includes_entry.fields.slug,
        title: includes_entry.fields.title,
        entry_title: includes_entry.fields.entry_title,
        text: includes_entry.fields.text,
        link: includes_entry.fields.link,
        data: includes_entry.fields.data.to_owned(),
        fallback_image: process_item(
            includes_entry.fields.fallback_image.to_owned(),
            key,
            includes.clone(),
        ),
        common_terms_and_conditions_items: process_item(
            includes_entry
                .fields
                .common_terms_and_conditions_items
                .to_owned(),
            key,
            includes.clone(),
        ),
        confirmation_text: includes_entry.fields.confirmation_text,
        error_text: includes_entry.fields.error_text,
        confirm_button_text: includes_entry.fields.confirm_button_text,
        file: includes_entry.fields.file.to_owned(),
        components: process_item(
            includes_entry.fields.components.to_owned(),
            key,
            includes.clone(),
        ),
        labels: process_item(
            includes_entry.fields.labels.to_owned(),
            key,
            includes.clone(),
        ),
        configs: process_item(
            includes_entry.fields.configs.to_owned(),
            key,
            includes.clone(),
        ),
        images: process_item(
            includes_entry.fields.images.to_owned(),
            key,
            includes,
        ),
    }
}

pub fn find_and_insert<'a>(
    link_type: &'a str,
    id: &'a str,
    key: &'a str,
    includes: Rc<ContentfulIncludes<'a>>,
) -> Option<ParsedIncludesEntry<'a>> {
    let entry: Option<&IncludesEntry<'a>> = includes.entries.iter().find(|&entry| entry.sys.id == id);
    match entry {
        Some(entry) => {
            if link_type == "Asset" {
                let parse_asset = ParsedIncludesAssetEntry {
                    slug: entry.fields.slug,
                    title: entry.fields.title,
                    text: entry.fields.text,
                    link: entry.fields.link,
                    data: entry.fields.data.to_owned(),
                    common_terms_and_conditions_items: entry
                        .fields
                        .common_terms_and_conditions_items
                        .to_owned(),
                    confirmation_text: entry.fields.confirmation_text,
                    error_text: entry.fields.error_text,
                    confirm_button_text: entry.fields.confirm_button_text,
                    url: "",
                    // url: entry
                    //     .fields
                    //     .file
                    //     .as_ref()
                    //     .and_then(|file| file.url),
                    file: entry.fields.file.to_owned(),
                };
                return Some(ParsedIncludesEntry::Asset(parse_asset));
            } else {
                let record = parse_data(entry, key, Rc::clone(&includes));
                // return Some(ParsedIncludesEntry::Entry(record));
                return None;
            }
        }
        None => None,
    }
}

pub fn normalize_labels<'a>(
    items: Rc<&ItemEntry<'a>>,
    includes: Rc<ContentfulIncludes<'a>>,
) -> HashMap<String, &'a str> {
    let mut record: HashMap<String, &'a str> = HashMap::new();
    let labels = &items.fields.labels;
    match labels {
        Item::Single(label) => {
            let entry = includes
                .entries
                .iter()
                .find(|entry| entry.sys.id == label.sys.id);
            match entry {
                Some(entry) => {
                    record.insert(to_camel_case(entry.fields.slug), entry.fields.text);
                }
                None => {}
            }
        }
        Item::Multiple(labels) => {
            for label in labels {
                let entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == label.sys.id);
                match entry {
                    Some(entry) => {
                        record.insert(to_camel_case(entry.fields.slug), entry.fields.text);
                    }
                    None => {}
                }
            }
        }
    }
    record
}

pub fn normalize_configs<'a>(
    items: Rc<&ItemEntry<'a>>,
    includes: Rc<ContentfulIncludes<'a>>,
) -> HashMap<String, Data> {
    let mut record: HashMap<String, Data> = HashMap::new();
    let configs = &items.fields.configs;
    match configs {
        Item::Single(config) => {
            let entry = includes
                .entries
                .iter()
                .find(|entry| entry.sys.id == config.sys.id);
            match entry {
                Some(entry) => {
                    record.insert(
                        to_camel_case(entry.fields.slug),
                        entry.fields.data.to_owned(),
                    );
                }
                None => {}
            }
        }
        Item::Multiple(configs) => {
            for config in configs {
                let entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == config.sys.id);
                match entry {
                    Some(entry) => {
                        record.insert(
                            to_camel_case(entry.fields.slug),
                            entry.fields.data.to_owned(),
                        );
                    }
                    None => {}
                }
            }
        }
    }
    record
}

pub fn normalize_components<'a: 'b, 'b>(
    items: Rc<&ItemEntry<'a>>,
    includes: Rc<ContentfulIncludes<'a>>,
) -> HashMap<String, Vec<ParsedIncludesEntry<'a>>> {
    let mut record: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    let components = &items.fields.components;
    match components {
        Item::Single(component) => {
            let entry = includes
                .entries
                .iter()
                .find(|entry| entry.sys.id == component.sys.id);
            match entry {
                Some(entry) => {
                    record.insert(
                        to_camel_case(entry.fields.slug),
                        process_item(
                            Item::Single(component.clone()),
                            "components",
                            Rc::clone(&includes),
                        ),
                    );
                }
                None => {}
            }
        }
        Item::Multiple(components) => {
            for component in components {
                let entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == component.sys.id);
                match entry {
                    Some(entry) => {
                        record.insert(
                            to_camel_case(entry.fields.slug),
                            process_item(
                                Item::Single(component.clone()),
                                "components",
                                Rc::clone(&includes),
                            ),
                        );
                    }
                    None => {}
                }
            }
        }
    }
    return record;
}

pub fn normalize_response<'a>(
    response: ContentfulResponse<'a>,
    slug: String,
) -> NormalizeResponseResult<'a> {
    let main_page_entry = response.items.iter().find(|item| item.fields.slug == slug);
    let includes: Rc<ContentfulIncludes<'a>> = Rc::new(response.includes);

    match main_page_entry {
        Some(main_page_entry) => {
            let items: Rc<&ItemEntry<'a>> = Rc::new(main_page_entry);
            NormalizeResponseResult {
                slug: main_page_entry.fields.slug,
                labels: normalize_labels(Rc::clone(&items), Rc::clone(&includes)),
                configs: normalize_configs(Rc::clone(&items), Rc::clone(&includes)),
                components: normalize_components(Rc::clone(&items), Rc::clone(&includes)),
            }
        }
        None => NormalizeResponseResult {
            slug: "",
            labels: HashMap::new(),
            configs: HashMap::new(),
            components: HashMap::new(),
        },
    }
}

#[cfg(test)]
mod tests;
