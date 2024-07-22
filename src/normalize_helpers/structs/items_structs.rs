use super::common_structs::ChildSys;
use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemsFields {
    pub slug: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub seo: Option<serde_json::Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ChildSys>>,
}
