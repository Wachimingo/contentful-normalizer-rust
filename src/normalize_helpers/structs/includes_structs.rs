use std::collections::HashMap;

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::{Number, Value};
use std::fmt;

use super::{common_structs::ChildSys, items_structs::Item};

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

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum IncludesFieldTypes<'a> {
//     Slug(String),
//     Text(String),
//     Link(String),
//     Data(Data),
//     FallbackImage(ChildSys<'a>),
//     CommonTermsAndConditionsItems(Item<ChildSys<'a>>),
//     ConfirmationText(String),
//     ErrorText(String),
//     ConfirmButtonText(String),
//     File(File),
//     Item(Item<ChildSys<'a>>),
//     Components(Item<ChildSys<'a>>),
//     Configs(Item<ChildSys<'a>>),
//     Labels(Item<ChildSys<'a>>),
//     Images(Item<ChildSys<'a>>),
// }

#[derive(Clone, Debug, Serialize)]
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
    pub fallback_image: Option<Item<ChildSys<'a>>>,
    #[serde(
        rename = "commonTermsAndConditionsItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_terms_and_conditions_items: Option<Item<ChildSys<'a>>>,
    #[serde(rename = "confirmationText", skip_serializing_if = "Option::is_none")]
    pub confirmation_text: Option<&'a str>,
    #[serde(rename = "errorText", skip_serializing_if = "Option::is_none")]
    pub error_text: Option<&'a str>,
    #[serde(rename = "confirmButtonText", skip_serializing_if = "Option::is_none")]
    pub confirm_button_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Item<ChildSys<'a>>>,
    #[serde(rename = "targetViewport", skip_serializing_if = "Option::is_none")]
    pub target_view_port: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'de: 'a, 'a> Deserialize<'de> for IncludesFields<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncludesFieldsVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for IncludesFieldsVisitor<'a> {
            type Value = IncludesFields<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IncludesFields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<IncludesFields<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut slug = None;
                let mut title = None;
                let mut entry_title = None;
                let mut text = None;
                let mut link = None;
                let mut data = None;
                let mut fallback_image = None;
                let mut common_terms_and_conditions_items = None;
                let mut confirmation_text = None;
                let mut error_text = None;
                let mut confirm_button_text = None;
                let mut file = None;
                let mut components = None;
                let mut labels = None;
                let mut configs = None;
                let mut images = None;
                let mut target_view_port = None;
                let mut description = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "slug" => {
                            if slug.is_some() {
                                return Err(de::Error::duplicate_field("slug"));
                            }
                            slug = Some(map.next_value()?);
                        }
                        "title" => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                        "entryTitle" => {
                            if entry_title.is_some() {
                                return Err(de::Error::duplicate_field("entryTitle"));
                            }
                            entry_title = Some(map.next_value()?);
                        }
                        "text" => {
                            if text.is_some() {
                                return Err(de::Error::duplicate_field("text"));
                            }
                            text = Some(map.next_value()?);
                        }
                        "link" => {
                            if link.is_some() {
                                return Err(de::Error::duplicate_field("link"));
                            }
                            link = Some(map.next_value()?);
                        }
                        "data" => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        "fallbackImage" => {
                            if fallback_image.is_some() {
                                return Err(de::Error::duplicate_field("fallbackImage"));
                            }
                            fallback_image = Some(map.next_value()?);
                        }
                        "commonTermsAndConditionsItems" => {
                            if common_terms_and_conditions_items.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "commonTermsAndConditionsItems",
                                ));
                            }
                            common_terms_and_conditions_items = Some(map.next_value()?);
                        }
                        "confirmationText" => {
                            if confirmation_text.is_some() {
                                return Err(de::Error::duplicate_field("confirmationText"));
                            }
                            confirmation_text = Some(map.next_value()?);
                        }
                        "errorText" => {
                            if error_text.is_some() {
                                return Err(de::Error::duplicate_field("errorText"));
                            }
                            error_text = Some(map.next_value()?);
                        }
                        "confirmButtonText" => {
                            if confirm_button_text.is_some() {
                                return Err(de::Error::duplicate_field("confirmButtonText"));
                            }
                            confirm_button_text = Some(map.next_value()?);
                        }
                        "file" => {
                            if file.is_some() {
                                return Err(de::Error::duplicate_field("file"));
                            }
                            file = Some(map.next_value()?);
                        }
                        "components" => {
                            if components.is_some() {
                                return Err(de::Error::duplicate_field("components"));
                            }
                            components = Some(map.next_value()?);
                        }
                        "labels" => {
                            if labels.is_some() {
                                return Err(de::Error::duplicate_field("labels"));
                            }
                            labels = Some(map.next_value()?);
                        }
                        "configs" => {
                            if configs.is_some() {
                                return Err(de::Error::duplicate_field("configs"));
                            }
                            configs = Some(map.next_value()?);
                        }
                        "images" => {
                            if images.is_some() {
                                return Err(de::Error::duplicate_field("images"));
                            }
                            images = Some(map.next_value()?);
                        }
                        "targetViewport" => {
                            if target_view_port.is_some() {
                                return Err(de::Error::duplicate_field("targetViewport"));
                            }
                            target_view_port = Some(map.next_value()?);
                        }
                        "description" => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        // _ => {
                        //     map.next_value()?;
                        // }
                    }
                }
                let slug = slug.unwrap_or_else(|| None);
                let title = title.unwrap_or_else(|| None);
                let entry_title =
                    entry_title.unwrap_or_else(|| None);
                let text = text.unwrap_or_else(|| None);
                let link = link.unwrap_or_else(|| None);
                let data = data.unwrap_or_else(|| None);
                let fallback_image =
                    fallback_image.unwrap_or_else(|| None);
                let common_terms_and_conditions_items = common_terms_and_conditions_items
                .unwrap_or_else(|| None);
                let confirmation_text = confirmation_text
                .unwrap_or_else(|| None);
                let error_text =
                    error_text.unwrap_or_else(|| None);
                let confirm_button_text = confirm_button_text
                .unwrap_or_else(|| None);
                let file = file.unwrap_or_else(|| None);
                let components =
                    components.unwrap_or_else(|| None);
                let labels = labels.unwrap_or_else(|| None);
                let configs = configs.unwrap_or_else(|| None);
                let images = images.unwrap_or_else(|| None);
                let target_view_port = target_view_port.unwrap_or_else(|| None);
                let description = description.unwrap_or_else(|| None);

                Ok(IncludesFields {
                    slug,
                    title,
                    entry_title,
                    text,
                    link,
                    data,
                    fallback_image,
                    common_terms_and_conditions_items,
                    confirmation_text,
                    error_text,
                    confirm_button_text,
                    file,
                    components,
                    labels,
                    configs,
                    images,
                    target_view_port,
                    description,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "slug",
            "title",
            "entry_title",
            "text",
            "link",
            "data",
            "fallback_image",
            "common_terms_and_conditions_items",
            "confirmation_text",
            "error_text",
            "confirm_button_text",
            "file",
            "components",
            "labels",
            "configs",
            "images",
            "target_view_port",
            "description",
        ];
        deserializer.deserialize_struct(
            "IncludesFields",
            FIELDS,
            IncludesFieldsVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

// // This is the way to make an struct be able to iter as a hashmap does
// impl IntoIterator for IncludesFields {
//     type Item = (String, Option<IncludesFieldTypes>);
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         vec![
//             ("slug".to_string(), self.slug.map(IncludesFieldTypes::Text)),
//             (
//                 "title".to_string(),
//                 self.title.map(IncludesFieldTypes::Text),
//             ),
//             (
//                 "entryTitle".to_string(),
//                 self.entry_title.map(IncludesFieldTypes::Text),
//             ),
//             ("text".to_string(), self.text.map(IncludesFieldTypes::Text)),
//             ("link".to_string(), self.link.map(IncludesFieldTypes::Link)),
//             ("data".to_string(), self.data.map(IncludesFieldTypes::Data)),
//             (
//                 "fallback_image".to_string(),
//                 self.fallback_image.map(IncludesFieldTypes::Item),
//             ),
//             (
//                 "common_terms_and_conditions_items".to_string(),
//                 self.common_terms_and_conditions_items
//                     .map(IncludesFieldTypes::CommonTermsAndConditionsItems),
//             ),
//             (
//                 "confirmation_text".to_string(),
//                 self.confirmation_text
//                     .map(IncludesFieldTypes::ConfirmationText),
//             ),
//             (
//                 "error_text".to_string(),
//                 self.error_text.map(IncludesFieldTypes::ErrorText),
//             ),
//             (
//                 "confirm_button_text".to_string(),
//                 self.confirm_button_text
//                     .map(IncludesFieldTypes::ConfirmButtonText),
//             ),
//             ("file".to_string(), self.file.map(IncludesFieldTypes::File)),
//             (
//                 "components".to_string(),
//                 self.components.map(IncludesFieldTypes::Item),
//             ),
//             (
//                 "labels".to_string(),
//                 self.labels.map(IncludesFieldTypes::Item),
//             ),
//             (
//                 "configs".to_string(),
//                 self.configs.map(IncludesFieldTypes::Item),
//             ),
//             (
//                 "images".to_string(),
//                 self.images.map(IncludesFieldTypes::Item),
//             ),
//         ]
//         .into_iter()
//     }
// }
