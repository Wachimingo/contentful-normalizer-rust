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

pub type IncludesEntries = Vec<IncludesEntry>;
pub type IncludesAssets = Vec<IncludesEntry>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulIncludes {
    pub entries: IncludesEntries,
    pub assets: IncludesAssets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulItems {
    pub entries: Vec<ItemEntry>,
}
