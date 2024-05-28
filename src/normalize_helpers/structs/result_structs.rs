use includes_structs::{CommonTermsAndConditionsItems, Data, File};
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParsedIncludesAssetEntry {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub link: Option<String>,
    pub data: Option<Data>,
    #[serde(rename = "commonTermsAndConditionsItems")]
    pub common_terms_and_conditions_items: Option<CommonTermsAndConditionsItems>,
    #[serde(rename = "confirmationText")]
    pub confirmation_text: Option<String>,
    #[serde(rename = "errorText")]
    pub error_text: Option<String>,
    #[serde(rename = "confirmButtonText")]
    pub confirm_button_text: Option<String>,    
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ParsedIncludesEntry {
    Asset(ParsedIncludesAssetEntry),
    Entry(IncludesFields),
}