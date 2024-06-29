use structs::{
    common_structs::ChildSys,
    includes_structs::Data,
    result_structs::{NormalizeResponseResult, ParsedFieldsResult, ParsedIncludesEntryResult},
    ContentfulResponse, IncludesEntry,
};

use crate::string_helpers::to_camel_case;
use std::collections::HashMap;
pub mod structs;
use self::structs::{
    items_structs::Item,
    result_structs::{ParsedIncludesAssetEntry, ParsedIncludesEntry},
    ContentfulIncludes,
};

pub fn process_item<'a>(
    item: Item<ChildSys<'a>>,
    key: &'a str,
    includes: &ContentfulIncludes<'a>,
) -> Vec<ParsedIncludesEntry<'a>> {
    let mut items_collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    // match item {
    //     Some(item) => match item {
    //         Item::Single(item) => {
    //             find_and_insert(
    //                 item.sys.unwrap().link_type,
    //                 item.sys.unwrap().id,
    //                 &key,
    //                 &includes,
    //                 &mut items_collector,
    //             );
    //         }
    //         Item::Multiple(items) => {
    //             for item in items {
    //                 find_and_insert(
    //                     item.sys.unwrap().link_type,
    //                     item.sys.unwrap().id,
    //                     &key,
    //                     &includes,
    //                     &mut items_collector,
    //                 );
    //             }
    //         }
    //     },
    //     None => {}
    // }
    let item_arr = items_collector
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<ParsedIncludesEntry>>();
    item_arr
}

pub fn find_data<'a>(
    includes_entry: &'a IncludesEntry<'a>,
    key: &'a str,
    includes: &ContentfulIncludes<'a>,
) -> ParsedIncludesEntryResult<'a> {
    let parsed_result = ParsedIncludesEntryResult {
        slug: includes_entry.fields.slug.clone(),
        title: includes_entry.fields.title.clone(),
        entry_title: includes_entry.fields.entry_title.clone(),
        text: includes_entry.fields.text.clone(),
        link: includes_entry.fields.link.clone(),
        data: includes_entry.fields.data.clone(),
        fallback_image: process_item(includes_entry.fields.fallback_image.clone(), key, includes),
        common_terms_and_conditions_items: process_item(
            includes_entry
                .fields
                .common_terms_and_conditions_items
                .clone(),
            key,
            includes,
        ),
        confirmation_text: includes_entry.fields.confirmation_text.clone(),
        error_text: includes_entry.fields.error_text.clone(),
        confirm_button_text: includes_entry.fields.confirm_button_text.clone(),
        file: includes_entry.fields.file.clone(),
        components: process_item(includes_entry.fields.components.clone(), key, includes),
        labels: process_item(includes_entry.fields.labels.clone(), key, includes),
        configs: process_item(includes_entry.fields.configs.clone(), key, includes),
        images: process_item(includes_entry.fields.images.clone(), key, includes),
    };
    parsed_result
}

pub fn find_and_insert<'a>(
    link_type: &'a str,
    id: &'a str,
    key: &'a str,
    includes: &'a ContentfulIncludes<'a>,
    collector: &'a mut HashMap<String, Vec<ParsedIncludesEntry<'a>>>,
) -> &'a HashMap<String, Vec<ParsedIncludesEntry<'a>>> {
    if link_type == "Asset" {
        let found_includes_entry = includes.entries.iter().find(|entry| entry.sys.id == id);
        match found_includes_entry {
            Some(found_includes_entry) => {
                let parse_asset = ParsedIncludesAssetEntry {
                    slug: found_includes_entry.fields.slug.clone(),
                    title: found_includes_entry.fields.title.clone(),
                    text: found_includes_entry.fields.text.clone(),
                    link: found_includes_entry.fields.link.clone(),
                    data: found_includes_entry.fields.data.clone(),
                    common_terms_and_conditions_items: found_includes_entry
                        .fields
                        .common_terms_and_conditions_items
                        .clone(),
                    confirmation_text: found_includes_entry.fields.confirmation_text.clone(),
                    error_text: found_includes_entry.fields.error_text.clone(),
                    confirm_button_text: found_includes_entry.fields.confirm_button_text.clone(),
                    url: "",
                    // url: found_includes_entry
                    //     .fields
                    //     .file
                    //     .as_ref()
                    //     .and_then(|file| file.url.clone()),
                    file: found_includes_entry.fields.file.clone(),
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
            None => {}
        }
    } else {
        let found_includes_entry = includes.entries.iter().find(|entry| entry.sys.id == id);
        match found_includes_entry {
            Some(found_includes_entry) => {
                match collector.get_mut(key) {
                    Some(arr) => {
                        let record = find_data(found_includes_entry, key, includes);
                        arr.push(ParsedIncludesEntry::Entry(record));
                    }
                    None => {
                        let record = find_data(found_includes_entry, key, includes);
                        collector.insert(key.to_string(), vec![ParsedIncludesEntry::Entry(record)]);
                    }
                };
            }
            None => {}
        }
    }
    collector
}

pub fn parse_fields<'a>(
    entry: IncludesEntry<'a>,
    includes: &'a ContentfulIncludes<'a>,
) -> ParsedFieldsResult<'a> {
    ParsedFieldsResult {
        title: entry.fields.title,
        slug: entry.fields.slug,
        components: process_item(entry.fields.components, "components", includes),
        labels: process_item(entry.fields.labels, "labels", includes),
        configs: process_item(entry.fields.configs, "configs", includes),
    }
}

pub fn normalize_labels<'a>(
    labels: &Item<ChildSys<'a>>,
    includes: &ContentfulIncludes<'a>,
) -> HashMap<String, &'a str> {
    let mut record: HashMap<String, &'a str> = HashMap::new();
    match labels {
        Item::Single(label) => {
            let found_includes_entry = includes
                .entries
                .iter()
                .find(|entry| entry.sys.id == label.sys.id);
            match found_includes_entry {
                Some(found_includes_entry) => {
                    record.insert(
                        to_camel_case(found_includes_entry.fields.slug),
                        found_includes_entry.fields.text,
                    );
                }
                None => {}
            }
        }
        Item::Multiple(labels) => {
            for label in labels {
                let found_includes_entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == label.sys.id);
                match found_includes_entry {
                    Some(found_includes_entry) => {
                        record.insert(
                            to_camel_case(found_includes_entry.fields.slug),
                            found_includes_entry.fields.text,
                        );
                    }
                    None => {}
                }
            }
        }
    }
    record
}

pub fn normalize_configs<'a>(
    configs: &Item<ChildSys<'a>>,
    includes: &'a ContentfulIncludes<'a>,
) -> HashMap<String, &'a Data> {
    let mut record: HashMap<String, &Data> = HashMap::new();
    match configs {
        Item::Single(config) => {
            let found_includes_entry = includes
                .entries
                .iter()
                .find(|entry| entry.sys.id == config.sys.id);
            match found_includes_entry {
                Some(found_includes_entry) => {
                    record.insert(
                        to_camel_case(found_includes_entry.fields.slug),
                        &found_includes_entry.fields.data,
                    );
                }
                None => {}
            }
        }
        Item::Multiple(configs) => {
            for config in configs {
                let found_includes_entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == config.sys.id);
                match found_includes_entry {
                    Some(found_includes_entry) => {
                        record.insert(
                            to_camel_case(found_includes_entry.fields.slug),
                            &found_includes_entry.fields.data,
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
    components: &Item<ChildSys<'a>>,
    includes: &'b ContentfulIncludes<'a>,
) -> HashMap<String, Vec<ParsedIncludesEntry<'a>>> {
    let mut record: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
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
                        process_item(Item::Single(component.clone()), "components", &includes),
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
                            process_item(Item::Single(component.clone()), "components", &includes),
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
    match main_page_entry {
        Some(main_page_entry) => {
            NormalizeResponseResult {
                slug: main_page_entry.fields.slug,
                labels: normalize_labels(&main_page_entry.fields.labels, &response.includes),
                // configs: normalize_configs(
                //     &main_page_entry.fields.configs,
                //     &response.includes,
                // ),
                configs: HashMap::new(),
                components: HashMap::new(),
                // components: Some(normalize_components(
                //     &main_page_entry.fields.components,
                //     &response.includes,
                // )),
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
