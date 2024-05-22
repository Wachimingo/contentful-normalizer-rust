use serde::{Deserialize, Serialize};
use super::*;

#[derive(Clone, Deserialize, Serialize)]
pub struct ParsedIncludesAssetEntry {
    pub sys: TopLevelSys,
    pub fields: IncludesFields,
    pub url: Option<String>,
}

pub enum ParsedIncludesEntry {
    Asset(ParsedIncludesAssetEntry),
    Entry(IncludesEntry),
}