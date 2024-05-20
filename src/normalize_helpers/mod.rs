#![allow(dead_code, unused_variables, unused_mut)]
use crate::string_helpers::to_camel_case;
use serde_json::Value;
use std::collections::HashMap;
mod structs;
use self::structs::{ContentfulEntity, ContentfulIncludes, Entry};

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

// pub fn parse_fields(entry: Entry, includes: ContentfulIncludes){
//     let mut parse_fields: HashMap<String, Value> = HashMap::new();
//     for (key, value) in entry.fields.iter() {
//         match value {
//             Value::Object(object) => {
//                 if object["sys"]["linkType"] == "Asset" {
                    
//                 }
//             },
//             // Value::Array(v) => {}
//             none => {}
//         }
//     };
// }

pub fn parse_fields(entry: Entry, includes: ContentfulIncludes){
    let mut parse_fields: HashMap<String, Value> = HashMap::new();
    for (key, value) in entry.fields.into_iter() {
        match value {
            Some(value) => {
                match value {             
                    _ => ()
                }
            },
            None => ()
        }
    };
}

#[cfg(test)]
mod tests;
