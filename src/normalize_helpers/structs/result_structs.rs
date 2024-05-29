use includes_structs::{CommonTermsAndConditionsItems, Data};
use serde::{Deserialize, Serializer, Serialize};
use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParsedIncludesAssetEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(rename = "commonTermsAndConditionsItems", skip_serializing_if = "Option::is_none")]
    pub common_terms_and_conditions_items: Option<CommonTermsAndConditionsItems>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<String>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<String>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ParsedIncludesEntry {
    Asset(ParsedIncludesAssetEntry),
    Entry(IncludesFields),
}

impl Serialize for ParsedIncludesEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParsedIncludesEntry::Asset(asset) => asset.serialize(serializer),
            ParsedIncludesEntry::Entry(entry) => entry.serialize(serializer),
        }
    }
}