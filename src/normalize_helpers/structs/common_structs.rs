use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChildSysInner {
    pub id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

// impl ChildSysInner {
//     pub fn get_id(&self) -> String {
//         self.id
//     }
//     pub fn get_link_type(&self) -> String {
//         self.link_type
//     }
//     pub fn get_object_type(&self) -> String {
//         self.object_type
//     }
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChildSys {
    pub sys: ChildSysInner
}

// impl ChildSys {
//     pub fn get_sys(&self) -> ChildSysInner {
//         self.sys
//     }
// }

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
    pub content_type: Option<ContentType>,
    pub locale: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulEntity {
    pub sys: TopLevelSys,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    tags: Option<Vec<String>>,
}