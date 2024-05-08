#![allow(dead_code)]
use crate::string_helpers::to_camel_case;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct Sys {
    pub id: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct ContentfulEntity {
    pub sys: Sys,
}
#[derive(Clone, Deserialize)]
pub struct Fields {
    pub slug: String,
    pub text: String,
    pub data: Vec<HashMap<String, Value>>,
}
#[derive(Clone, Deserialize)]
pub struct Entries {
    pub sys: Sys,
    pub fields: Fields,
}
#[derive(Clone, Deserialize)]
pub struct ContentfulIncludes {
    pub entries: Vec<Entries>,
    // pub assets: Vec<Sys>,
}

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

#[cfg(test)]
mod tests;
