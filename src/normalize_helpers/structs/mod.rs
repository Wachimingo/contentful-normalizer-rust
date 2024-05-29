use serde::{Deserialize, Serialize};
pub mod common_structs;
pub mod includes_structs;
pub mod items_structs;
pub mod result_structs;

use self::{
    common_structs::TopLevelSys, includes_structs::IncludesFields, items_structs::ItemsFields,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludesEntry {
    pub sys: TopLevelSys,
    pub fields: IncludesFields,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemEntry {
    pub sys: TopLevelSys,
    pub fields: ItemsFields,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulIncludes {
    #[serde(rename = "Entry")]
    pub entries: Vec<IncludesEntry>,
    #[serde(rename = "Asset")]
    pub assets: Vec<IncludesEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulItems {
    pub entries: Vec<ItemEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulResponse {
    // pub sys: ChildSysInner,
    pub total: u8,
    pub skip: u8,
    pub limit: u8,
    pub items: Vec<ItemEntry>,
    pub includes: ContentfulIncludes,
}
