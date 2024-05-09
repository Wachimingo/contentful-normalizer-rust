use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};

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