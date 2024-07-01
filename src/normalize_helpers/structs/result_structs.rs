use std::collections::HashMap;

use super::*;
use common_structs::ChildSys;
use includes_structs::{Data, File};
use items_structs::Item;
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct ParsedIncludesAssetEntry<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub text: &'a str,
    pub link: &'a str,
    pub data: Data,
    #[serde(rename = "commonTermsAndConditionsItems")]
    pub common_terms_and_conditions_items: Item<ChildSys<'a>>,
    #[serde(rename = "confirmationText")]
    pub confirmation_text: &'a str,
    #[serde(rename = "errorText")]
    pub error_text: &'a str,
    #[serde(rename = "confirmButtonText")]
    pub confirm_button_text: &'a str,
    pub url: &'a str,
    pub file: File<'a>,
}

#[derive(Clone, Debug)]
pub enum ParsedIncludesEntry<'a> {
    Asset(ParsedIncludesAssetEntry<'a>),
    Entry(ParsedIncludesEntryResult<'a>),
    Text(String),
}

impl<'a> Serialize for ParsedIncludesEntry<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParsedIncludesEntry::Asset(asset) => asset.serialize(serializer),
            ParsedIncludesEntry::Entry(entry) => entry.serialize(serializer),
            ParsedIncludesEntry::Text(entry) => entry.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ParsedIncludesEntryResult<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    #[serde(rename = "entryTitle")]
    pub entry_title: &'a str,
    pub text: &'a str,
    pub link: &'a str,
    pub data: Data,
    #[serde(rename = "fallbackImage")]
    pub fallback_image: Vec<ParsedIncludesEntry<'a>>,
    #[serde(rename = "commonTermsAndConditionsItems")]
    pub common_terms_and_conditions_items: Vec<ParsedIncludesEntry<'a>>,
    #[serde(rename = "confirmationText")]
    pub confirmation_text: &'a str,
    #[serde(rename = "errorText")]
    pub error_text: &'a str,
    #[serde(rename = "confirmButtonText")]
    pub confirm_button_text: &'a str,
    pub file: File<'a>,
    pub components: Vec<ParsedIncludesEntry<'a>>,
    pub labels: Vec<ParsedIncludesEntry<'a>>,
    pub configs: Vec<ParsedIncludesEntry<'a>>,
    pub images: Vec<ParsedIncludesEntry<'a>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParsedFieldsResult<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub components: Vec<ParsedIncludesEntry<'a>>,
    pub labels: Vec<ParsedIncludesEntry<'a>>,
    pub configs: Vec<ParsedIncludesEntry<'a>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NormalizeResponseResult<'a> {
    pub slug: &'a str,
    pub labels: HashMap<String, &'a str>,
    pub configs: HashMap<String, Data>,
    pub components: HashMap<String, Vec<ParsedIncludesEntry<'a>>>,
}
