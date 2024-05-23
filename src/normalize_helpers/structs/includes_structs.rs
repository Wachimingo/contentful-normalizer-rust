use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common_structs::ChildSys;

pub type CommonTermsAndConditionsItems = Vec<ChildSys>;
pub type Data = Vec<HashMap<String, Value>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub url: Option<String>,
}

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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludesFields {
    pub slug: String,
    pub text: Option<String>,
    pub link: Option<String>,
    pub data: Option<Data>,
    #[serde(rename = "fallbackImage")]
    pub fallback_image: Option<ChildSys>,
    #[serde(rename = "commonTermsAndConditionsItems")]
    pub common_terms_and_conditions_items: Option<CommonTermsAndConditionsItems>,
    #[serde(rename = "confirmationText")]
    pub confirmation_text: Option<String>,
    #[serde(rename = "errorText")]
    pub error_text: Option<String>,
    #[serde(rename = "confirmButtonText")]
    pub confirm_button_text: Option<String>,
    pub file: Option<File>,
}

// This is the way to make an struct be able to iter as a hashmap does
impl IntoIterator for IncludesFields {
    type Item = (String, Option<IncludesFieldTypes>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("text".to_string(), self.text.map(IncludesFieldTypes::Text)),
            ("link".to_string(), self.link.map(IncludesFieldTypes::Link)),
            (
                "slug".to_string(),
                Some(IncludesFieldTypes::Slug(self.slug)),
            ),
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
            (
                "file".to_string(),
                self.file
                    .map(IncludesFieldTypes::File),
            ),
        ]
        .into_iter()
    }
}
