use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ChildSys {
    pub id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Space {
    pub sys: ChildSys,    
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Environment {
    pub sys: ChildSys,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ContentType {
    pub sys: ChildSys,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TopLevelSys {
    pub id: String,
    pub space: Space,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub environment: Environment,
    pub revision: u8,
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    pub locale: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct ContentfulEntity {
    pub sys: TopLevelSys,
}
#[derive(Clone, Deserialize)]
pub struct Fields {
    pub slug: String,
    pub text: Option<String>,
    pub data: Option<Vec<HashMap<String, Value>>>,
}
#[derive(Clone, Deserialize)]
pub struct Entries {
    pub sys: TopLevelSys,
    pub fields: Fields,
}
#[derive(Clone, Deserialize)]
pub struct ContentfulIncludes {
    pub entries: Vec<Entries>,
    pub assets: Vec<TopLevelSys>,
}

#[derive(Clone, Deserialize)]
pub struct ContentfulEntry {
    pub fields: HashMap<String, Value>,
}