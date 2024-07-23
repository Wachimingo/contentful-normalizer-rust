use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use super::common_structs::ChildSys;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data {
    pub items: Vec<HashMap<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileDetailsImage {
    pub width: Number,
    pub height: Number,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileDetails {
    pub size: Number,
    pub image: FileDetailsImage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<FileDetails>,
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludesFields<'a> {
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
    pub data: Option<Data>,
    #[serde(rename = "fallbackImage", skip_serializing_if = "Option::is_none")]
    pub fallback_image: Option<ChildSys>,
    #[serde(
        rename = "commonTermsAndConditionsItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_terms_and_conditions_items: Option<Vec<ChildSys>>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<&'a str>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<&'a str>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ChildSys>>,
}