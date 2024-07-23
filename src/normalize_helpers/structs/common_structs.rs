use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChildSysInner {
    pub id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub object_type: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChildSys {
    pub sys: ChildSysInner
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Space {
    pub sys: ChildSysInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Environment {
    pub sys: ChildSysInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentType {
    pub sys: ChildSysInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopLevelSys<'a> {
    pub id: &'a str,
    pub space: Space,
    #[serde(rename = "type")]
    pub object_type: &'a str,
    #[serde(rename = "createdAt")]
    pub created_at: &'a str,
    #[serde(rename = "updatedAt")]
    pub updated_at: &'a str,
    pub environment: Environment,
    pub revision: u8,
    #[serde(rename = "contentType")]
    pub content_type: Option<ContentType>,
    pub locale: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    tags: Option<Vec<String>>,
}