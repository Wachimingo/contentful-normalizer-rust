use structs::{
    common_structs::{ChildSys, SysField},
    includes_structs::Data,
    items_structs::{ItemRef, SingleMultiple},
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

pub fn process_item<'a, U: SysField, V: IntoIterator<Item = U> + 'a, T: SingleMultiple<'a, U, V> + 'a>(
    item: &'a Option<T>,
    key: &str,
    includes: &ContentfulIncludes,
) -> Option<Vec<ParsedIncludesEntry>> {
    let mut items_collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    match item {
        Some(item) => {
            match item.single() {
                Some(item) => {
                    find_and_insert(
                        &item.link_type(),
                        &item.id(),
                        &key,
                        &includes,
                        &mut items_collector,
                    );
                }
                None => {}
            };
            match item.multiple() {
                Some(items) => {
                    for item in items {
                        find_and_insert(
                            &item.link_type(),
                            &item.id(),
                            &key,
                            &includes,
                            &mut items_collector,
                        );
                    }
                }
                None => {}
            };
        }
        None => {}
    }
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
        entry_title: includes_entry.fields.entry_title.clone(),
        text: includes_entry.fields.text.clone(),
        link: includes_entry.fields.link.clone(),
        data: includes_entry.fields.data.clone(),
        fallback_image: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&includes_entry.fields.fallback_image, key, includes),
        common_terms_and_conditions_items: process_item::<&ChildSys, Vec<&ChildSys>, Item>(
            &includes_entry.fields.common_terms_and_conditions_items.clone(),
            key,
            includes,
        ),
        confirmation_text: includes_entry.fields.confirmation_text.clone(),
        error_text: includes_entry.fields.error_text.clone(),
        confirm_button_text: includes_entry.fields.confirm_button_text.clone(),
        file: None,
        components: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&includes_entry.fields.components.clone(), key, includes),
        labels: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&includes_entry.fields.labels.clone(), key, includes),
        configs: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&includes_entry.fields.configs.clone(), key, includes),
        images: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&includes_entry.fields.images.clone(), key, includes),
    };
    parsed_result
}

pub fn find_and_insert<'vec>(
    link_type: &str,
    id: &str,
    key: &str,
    includes: &ContentfulIncludes,
    collector: &'vec mut HashMap<String, Vec<ParsedIncludesEntry>>,
) -> &'vec HashMap<String, Vec<ParsedIncludesEntry>> {
    if link_type == "Asset" {
        let found_includes_entry = &includes
            .entries
            .iter()
            .find(|entry| entry.sys.id == id);
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
                    url: found_includes_entry
                        .fields
                        .file
                        .as_ref()
                        .and_then(|file| file.url.clone()),
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
        let found_includes_entry = &includes
            .entries
            .iter()
            .find(|entry| entry.sys.id == id);
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

pub fn parse_fields(entry: IncludesEntry, includes: &ContentfulIncludes) -> ParsedFieldsResult {
    ParsedFieldsResult {
        title: entry.fields.title,
        slug: entry.fields.slug,
        components: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&entry.fields.components, "components", includes),
        labels: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&entry.fields.labels, "labels", includes),
        configs: process_item::<&ChildSys, Vec<&ChildSys>, Item>(&entry.fields.configs, "configs", includes),
    }
}

pub fn normalize_labels(
    labels: &Option<Item>,
    includes: &ContentfulIncludes,
) -> HashMap<String, String> {
    let mut record: HashMap<String, String> = HashMap::new();
    match labels {
        Some(labels) => {
            match labels {
                Item::Single(label) => {
                    let found_includes_entry = includes
                        .entries
                        .iter()
                        .find(|entry| entry.sys.id == label.sys.id);
                    match found_includes_entry {
                        Some(found_includes_entry) => match &found_includes_entry.fields.text {
                            Some(text) => {
                                record.insert(
                                    to_camel_case(found_includes_entry.fields.slug.as_ref()),
                                    text.clone(),
                                );
                            }
                            None => {}
                        },
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
                            Some(found_includes_entry) => match &found_includes_entry.fields.text {
                                Some(text) => {
                                    record.insert(
                                        to_camel_case(found_includes_entry.fields.slug.as_ref()),
                                        text.clone(),
                                    );
                                }
                                None => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            record
        }
        None => record,
    }
}

pub fn normalize_configs<'data>(
    configs: &Option<Item>,
    includes: &'data ContentfulIncludes,
) -> HashMap<String, &'data Data> {
    let mut record: HashMap<String, &Data> = HashMap::new();
    match configs {
        Some(configs) => {
            match configs {
                Item::Single(config) => {
                    let found_includes_entry = includes
                        .entries
                        .iter()
                        .find(|entry| entry.sys.id == config.sys.id);
                    match found_includes_entry {
                        Some(found_includes_entry) => match &found_includes_entry.fields.data {
                            Some(data) => {
                                record.insert(
                                    to_camel_case(found_includes_entry.fields.slug.as_ref()),
                                    data,
                                );
                            }
                            None => {}
                        },
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
                            Some(found_includes_entry) => match &found_includes_entry.fields.data {
                                Some(data) => {
                                    record.insert(
                                        to_camel_case(found_includes_entry.fields.slug.as_ref()),
                                        data,
                                    );
                                }
                                None => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            record
        }
        None => record,
    }
}

pub fn normalize_components(
    components: &Option<Item>,
    includes: &ContentfulIncludes,
) -> HashMap<String, Option<Vec<ParsedIncludesEntry>>> {
    let mut record: HashMap<String, Option<Vec<ParsedIncludesEntry>>> = HashMap::new();
    match components {
        Some(components) => match components {
            Item::Single(component) => {
                let entry = includes
                    .entries
                    .iter()
                    .find(|entry| entry.sys.id == component.sys.id);
                match entry {
                    Some(entry) => {
                        record.insert(
                            to_camel_case(entry.fields.slug.as_ref()),
                            process_item::<&ChildSys, &Vec<ChildSys>, ItemRef>(&Some(ItemRef::Single(component)), "components", &includes),
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
                                to_camel_case(entry.fields.slug.as_ref()),
                                process_item::<&ChildSys, &Vec<ChildSys>, ItemRef>(&Some(ItemRef::Single(component)), "components", &includes),
                            );
                        }
                        None => {}
                    }
                }
            }
        },
        None => {}
    }
    return record;
}

pub fn normalize_response<'slug, 'data>(response: ContentfulResponse<'slug>, slug: String) -> NormalizeResponseResult<'slug, 'data> {
    let main_page_entry = response.items.iter().find(|item| item.fields.slug == slug);
    match main_page_entry {
        Some(main_page_entry) => NormalizeResponseResult {
            slug: Some(main_page_entry.fields.slug),
            // labels: Some(normalize_labels(
            //     &main_page_entry.fields.labels,
            //     &response.includes,
            // )),
            labels: None,
            configs: None,
            // configs: Some(normalize_configs(
            //     &main_page_entry.fields.configs,
            //     &response.includes,
            // )),
            // components: Some(normalize_components(
            //     &main_page_entry.fields.components,
            //     &response.includes,
            // )),
            components: None,
        },
        None => NormalizeResponseResult {
            slug: None,
            labels: None,
            configs: None,
            components: None,
        },
    }
}

#[cfg(test)]
mod tests;
