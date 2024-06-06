#![allow(dead_code, unused_variables, unused_mut)]
use structs::result_structs::ParsedIncludesEntryResult;

use crate::string_helpers::to_camel_case;
use std::collections::HashMap;
pub mod structs;
use self::structs::{
    common_structs::ContentfulEntity,
    items_structs::{Item, ItemsFieldTypes},
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

// pub fn normalize_configs(
//     configs: Vec<ContentfulEntity>,
//     includes: ContentfulIncludes,
// ) -> HashMap<String, Vec<HashMap<String, Value>>> {
//     let mut record: HashMap<String, Vec<HashMap<String, Value>>> = HashMap::new();
//     for config in configs {
//         for entry in &includes.entries {
//             if config.sys.id == entry.sys.id {
//                 // This way allows for coping the struct values, compared to match statement where data couldn't be moved
//                 if let Some(ref data) = entry.fields.data {
//                     record.insert(to_camel_case(&entry.fields.slug), data.clone());
//                 }
//             }
//         }
//     }
//     return record;
// }

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
                        let mut components_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.components.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut components_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut components_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut labels_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.labels.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut labels_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut labels_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut configs_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.configs.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut configs_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut configs_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut images_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.images.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut images_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut images_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let components_arr = components_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let labels_arr = labels_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let configs_arr = configs_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let images_arr = images_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();

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
                            components: if components_arr.is_empty() {
                                None
                            } else {
                                Some(components_arr)
                            },
                            labels: if labels_arr.is_empty() {
                                None
                            } else {
                                Some(labels_arr)
                            },
                            configs: if configs_arr.is_empty() {
                                None
                            } else {
                                Some(configs_arr)
                            },
                            images: if images_arr.is_empty() {
                                None
                            } else {
                                Some(images_arr)
                            },
                        };
                        arr.push(ParsedIncludesEntry::ParsedIncludesEntryResult(
                            parsed_result,
                        ));
                    }
                    None => {
                        let mut components_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.components.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut components_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut components_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut labels_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.labels.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut labels_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut labels_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut configs_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.configs.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut configs_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut configs_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let mut images_collector: HashMap<String, Vec<ParsedIncludesEntry>> =
                            HashMap::new();
                        match includes_entry.fields.images.clone() {
                            Some(value) => match value {
                                Item::Single(item) => {
                                    find_and_insert(
                                        &item.sys.link_type,
                                        &item.sys.id,
                                        &key,
                                        &includes,
                                        &mut images_collector,
                                    );
                                }
                                Item::Multiple(items) => {
                                    for item in items {
                                        find_and_insert(
                                            &item.sys.link_type,
                                            &item.sys.id,
                                            &key,
                                            &includes,
                                            &mut images_collector,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }

                        let components_arr = components_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let labels_arr = labels_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let configs_arr = configs_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();
                        let images_arr = images_collector
                            .values()
                            .cloned()
                            .flatten()
                            .collect::<Vec<ParsedIncludesEntry>>();

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
                            components: if components_arr.is_empty() {
                                None
                            } else {
                                Some(components_arr)
                            },
                            labels: if labels_arr.is_empty() {
                                None
                            } else {
                                Some(labels_arr)
                            },
                            configs: if configs_arr.is_empty() {
                                None
                            } else {
                                Some(configs_arr)
                            },
                            images: if images_arr.is_empty() {
                                None
                            } else {
                                Some(images_arr)
                            },
                        };
                        collector.insert(
                            key.to_string(),
                            vec![ParsedIncludesEntry::ParsedIncludesEntryResult(
                                parsed_result,
                            )],
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
) -> HashMap<String, Vec<ParsedIncludesEntry>> {
    let mut collector: HashMap<String, Vec<ParsedIncludesEntry>> = HashMap::new();
    for (key, value) in entry.fields.into_iter() {
        match value {
            Some(value) => {
                match value {
                    ItemsFieldTypes::Item(value) => {
                        // if item field entry is component, image or config
                        match value {
                            Item::Single(value) => {
                                // if value is an object or array
                                find_and_insert(
                                    &value.sys.link_type,
                                    &value.sys.id,
                                    &key,
                                    &includes,
                                    &mut collector,
                                );
                            }
                            Item::Multiple(value) => {
                                for value_item in value {
                                    find_and_insert(
                                        &value_item.sys.link_type,
                                        &value_item.sys.id,
                                        &key,
                                        &includes,
                                        &mut collector,
                                    );
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            None => (),
        }
    }
    collector
}

#[cfg(test)]
mod tests;
