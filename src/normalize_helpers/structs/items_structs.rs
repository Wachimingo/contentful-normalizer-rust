use super::common_structs::ChildSys;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemsFields<'a> {
    pub slug: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ChildSys>>,
}
