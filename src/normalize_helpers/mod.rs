#![allow(dead_code, unused_variables, unused_mut)]
use crate::string_helpers::to_camel_case;
use serde_json::Value;
use std::collections::HashMap;
mod structs;
use self::structs::{
    common_structs::ContentfulEntity,
    items_structs::{Item, ItemsFieldTypes},
    ContentfulIncludes, IncludesEntry, ItemEntry,
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
) -> HashMap<String, Vec<HashMap<String, Value>>> {
    let mut record: HashMap<String, Vec<HashMap<String, Value>>> = HashMap::new();
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

pub fn find_and_insert(link_type: &str, id: &str, key: &str, includes: &ContentfulIncludes, parse_fields: &mut HashMap<String, IncludesEntry> ) -> (){
    if link_type == "Asset" {
        for asset in &includes.assets {
            if asset.sys.id == id {
                parse_fields.insert(key.to_string(), asset.clone());
            }
        }
    } else {
        for includes_entry in &includes.entries {
            if includes_entry.sys.id == id {
                parse_fields
                    .insert(key.to_string(), includes_entry.clone());
            }
        }
    }
}

pub fn parse_fields(entry: ItemEntry, includes: ContentfulIncludes) {
    let mut parse_fields: HashMap<String, IncludesEntry> = HashMap::new();
    for (key, value) in entry.fields.into_iter() {
        match value {
            Some(value) => {
                match value {
                    ItemsFieldTypes::Item(value) => {
                        // if item field entry is component, image or config
                        match value {
                            Item::Single(value) => {
                                // if value is an object or array
                                find_and_insert(&value.link_type, &value.id, &key, &includes, &mut parse_fields)
                            }
                            Item::Multiple(value) => {
                                for value_item in value {
                                    find_and_insert(&value_item.link_type, &value_item.id, &key, &includes, &mut parse_fields)
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
}

#[cfg(test)]
mod tests;
