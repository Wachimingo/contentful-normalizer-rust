#![allow(dead_code)]

use std::collections::HashMap;
use crate::string_helpers::to_camel_case;

#[derive(Clone)]
pub struct Sys {
    pub id: String,
}
#[derive(Clone)]
pub struct ContentfulEntity {
    pub sys: Sys
}
#[derive(Clone)]
pub struct Fields {
    pub slug: String,
    pub text: String,
}
#[derive(Clone)]
pub struct Entries {
    pub sys: Sys,
    pub fields: Fields
}
#[derive(Clone)]
pub struct ContentfulIncludes {
    pub entries: Vec<Entries>,
    // pub assets: Vec<Sys>,
}

pub fn normalize_labels(labels: Vec<ContentfulEntity>, includes: ContentfulIncludes)-> HashMap<String, String>{
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