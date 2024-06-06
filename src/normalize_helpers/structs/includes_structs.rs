use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{common_structs::ChildSys, items_structs::Item};

pub type CommonTermsAndConditionsItems = Vec<ChildSys>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data {
    pub items: Vec<HashMap<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub url: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IncludesFieldTypes {
    Slug(String),
    Text(String),
    Link(String),
    Data(Data),
    FallbackImage(ChildSys),
    CommonTermsAndConditionsItems(CommonTermsAndConditionsItems),
    ConfirmationText(String),
    ErrorText(String),
    ConfirmButtonText(String),
    File(File),
    Item(Item),
    Components(Item),
    Configs(Item),
    Labels(Item),
    Images(Item),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludesFields {
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
    #[serde(rename = "fallbackImage", skip_serializing_if = "Option::is_none")]
    pub fallback_image: Option<ChildSys>,
    #[serde(
        rename = "commonTermsAndConditionsItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_terms_and_conditions_items: Option<CommonTermsAndConditionsItems>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<String>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<String>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Item>,
}

// This is the way to make an struct be able to iter as a hashmap does
impl IntoIterator for IncludesFields {
    type Item = (String, Option<IncludesFieldTypes>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("slug".to_string(), self.slug.map(IncludesFieldTypes::Text)),
            (
                "title".to_string(),
                self.title.map(IncludesFieldTypes::Text),
            ),
            ("text".to_string(), self.text.map(IncludesFieldTypes::Text)),
            ("link".to_string(), self.link.map(IncludesFieldTypes::Link)),
            ("data".to_string(), self.data.map(IncludesFieldTypes::Data)),
            (
                "fallback_image".to_string(),
                self.fallback_image.map(IncludesFieldTypes::FallbackImage),
            ),
            (
                "common_terms_and_conditions_items".to_string(),
                self.common_terms_and_conditions_items
                    .map(IncludesFieldTypes::CommonTermsAndConditionsItems),
            ),
            (
                "confirmation_text".to_string(),
                self.confirmation_text
                    .map(IncludesFieldTypes::ConfirmationText),
            ),
            (
                "error_text".to_string(),
                self.error_text.map(IncludesFieldTypes::ErrorText),
            ),
            (
                "confirm_button_text".to_string(),
                self.confirm_button_text
                    .map(IncludesFieldTypes::ConfirmButtonText),
            ),
            ("file".to_string(), self.file.map(IncludesFieldTypes::File)),
            (
                "components".to_string(),
                self.components.map(IncludesFieldTypes::Item),
            ),
            (
                "labels".to_string(),
                self.labels.map(IncludesFieldTypes::Item),
            ),
            (
                "configs".to_string(),
                self.configs.map(IncludesFieldTypes::Item),
            ),
            (
                "images".to_string(),
                self.images.map(IncludesFieldTypes::Item),
            ),
        ]
        .into_iter()
    }
}
