use serde::{Deserialize, Serialize};
pub mod common_structs;
pub mod includes_structs;
pub mod items_structs;

use self::{common_structs::TopLevelSys, includes_structs::IncludesFields, items_structs::ItemsFields};

#[derive(Clone, Deserialize, Serialize)]
pub struct IncludesEntry {
    pub sys: TopLevelSys,
    pub fields: IncludesFields,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ItemEntry {
    pub sys: TopLevelSys,
    pub fields: ItemsFields,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ContentfulIncludes {
    pub entries: Vec<IncludesEntry>,
    pub assets: Vec<IncludesEntry>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ContentfulItems {
    pub entries: Vec<ItemEntry>,
}