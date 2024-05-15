#![allow(dead_code, unused_variables, unused_mut)]
use crate::string_helpers::to_camel_case;
use serde_json::Value;
use std::collections::HashMap;
mod structs;
use self::structs::{ContentfulEntity, ContentfulEntry, ContentfulIncludes};

pub fn normalize_labels(
    labels: Vec<ContentfulEntity>,
    includes: ContentfulIncludes,
) -> HashMap<String, String> {
    let mut record: HashMap<String, String> = HashMap::new();
    for label in labels {
        for entry in &includes.entries {
            if label.sys.id == entry.sys.id {
                record.insert(to_camel_case(&entry.fields.slug), entry.fields.text.clone());
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
                record.insert(to_camel_case(&entry.fields.slug), entry.fields.data.clone());
            }
        }
    }
    return record;
}

pub fn parse_fields(entry: ContentfulEntry, includes: ContentfulIncludes){
    let mut parse_fields: HashMap<String, Value> = HashMap::new();
    for (key, value) in entry.fields.iter() {
        match value {
            Value::Object(object) => {
                if object["sys"]["linkType"] == "Asset" {
                    
                }
            },
            // Value::Array(v) => {}
            none => {}
        }
    };
}

#[cfg(test)]
mod tests;
