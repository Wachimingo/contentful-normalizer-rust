use std::collections::HashMap;

use super::*;
use common_structs::ChildSys;
use includes_structs::{Data, File};
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct ParsedIncludesAssetEntry<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a Data>,
    #[serde(
        rename = "commonTermsAndConditionsItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_terms_and_conditions_items: Option<&'a Vec<ChildSys>>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<&'a str>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<&'a str>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<&'a File<'a>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(rename = "entryTitle", skip_serializing_if = "Option::is_none")]
    pub entry_title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a Data>,
    #[serde(rename = "fallbackImage", skip_serializing_if = "Option::is_none")]
    pub fallback_image: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(
        rename = "commonTermsAndConditionsItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_terms_and_conditions_items: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<&'a str>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<&'a str>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ParsedIncludesEntry<'a>>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParsedFieldsResult<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ParsedIncludesEntry<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ParsedIncludesEntry<'a>>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NormalizeResponseResult<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<HashMap<String, &'a Data>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<HashMap<String, Option<Vec<ParsedIncludesEntry<'a>>>>>,
}
