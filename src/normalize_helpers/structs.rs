use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct ChildSys {
    pub id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Space {
    pub sys: ChildSys,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Environment {
    pub sys: ChildSys,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ContentType {
    pub sys: ChildSys,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TopLevelSys {
    pub id: String,
    pub space: Space,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub environment: Environment,
    pub revision: u8,
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    pub locale: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct ContentfulEntity {
    pub sys: TopLevelSys,
}

pub type Labels = Vec<ChildSys>;
pub type Configs = Vec<ChildSys>;
pub type Images = Vec<ChildSys>;
pub type CommonTermsAndConditionsItems = Vec<ChildSys>;
pub type Data = Vec<HashMap<String, Value>>;

#[derive(Clone, Deserialize)]
pub struct Fields {
    pub slug: String,
    pub text: Option<String>,
    pub link: Option<String>,
    pub data: Option<Data>,
    pub labels: Option<Labels>,
    pub configs: Option<Configs>,
    pub images: Option<Images>,
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
}

pub enum FieldItems {
    Slug(String),
    Text(String),
    Link(String),
    Data(Data),
    Labels(Labels),
    Configs(Configs),
    Images(Images),
    FallbackImage(ChildSys),
    CommonTermsAndConditionsItems(CommonTermsAndConditionsItems),
    ConfirmationText(String),
    ErrorText(String),
    ConfirmButtonText(String),
}

impl IntoIterator for Fields {
    type Item = (String, Option<FieldItems>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("text".to_string(), self.text.map(FieldItems::Text)),
            ("link".to_string(), self.link.map(FieldItems::Link)),
            ("slug".to_string(), Some(FieldItems::Slug(self.slug))),
            ("data".to_string(), self.data.map(FieldItems::Data)),
            ("labels".to_string(), self.labels.map(FieldItems::Labels)),
            (
                "configs".to_string(),
                self.configs.map(FieldItems::Configs),
            ),
            (
                "fallback_image".to_string(),
                self.fallback_image.map(FieldItems::FallbackImage),
            ),
            (
                "common_terms_and_conditions_items".to_string(),
                self.common_terms_and_conditions_items
                    .map(FieldItems::CommonTermsAndConditionsItems),
            ),
            (
                "confirmation_text".to_string(),
                self.confirmation_text.map(FieldItems::ConfirmationText),
            ),
            (
                "error_text".to_string(),
                self.error_text.map(FieldItems::ErrorText),
            ),
            (
                "confirm_button_text".to_string(),
                self.confirm_button_text.map(FieldItems::ConfirmButtonText),
            ),
        ]
        .into_iter()
    }
}

#[derive(Clone, Deserialize)]
pub struct Entry {
    pub sys: TopLevelSys,
    pub fields: Fields,
}
#[derive(Clone, Deserialize)]
pub struct ContentfulIncludes {
    pub entries: Vec<Entry>,
    pub assets: Vec<TopLevelSys>,
}
