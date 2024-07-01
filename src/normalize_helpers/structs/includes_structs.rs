use std::collections::HashMap;

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::{Number, Value};
use std::fmt;

use crate::normalize_helpers::structs::common_structs::ChildSysInner;

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
pub struct File<'a> {
    #[serde(default)]
    pub url: &'a str,
    pub details: FileDetails,
    #[serde(default)]
    pub file_name: &'a str,
    #[serde(default)]
    pub content_type: &'a str,
}

#[derive(Clone, Debug, Serialize)]
pub struct IncludesFields<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    #[serde(rename = "entryTitle")]
    pub entry_title: &'a str,
    pub text: &'a str,
    pub link: &'a str,
    pub data: Data,
    #[serde(rename = "fallbackImage")]
    pub fallback_image: Item<ChildSys<'a>>,
    #[serde(rename = "commonTermsAndConditionsItems")]
    pub common_terms_and_conditions_items: Item<ChildSys<'a>>,
    #[serde(rename = "confirmationText")]
    pub confirmation_text: &'a str,
    #[serde(rename = "errorText")]
    pub error_text: &'a str,
    #[serde(rename = "confirmButtonText")]
    pub confirm_button_text: &'a str,
    pub file: File<'a>,
    pub components: Item<ChildSys<'a>>,
    pub labels: Item<ChildSys<'a>>,
    pub configs: Item<ChildSys<'a>>,
    pub images: Item<ChildSys<'a>>,
    #[serde(rename = "targetViewport")]
    pub target_view_port: &'a str,
    pub description: &'a str,
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
                let mut slug = "";
                let mut title = "";
                let mut entry_title = "";
                let mut text = "";
                let mut link = "";
                let mut data = Data { items: Vec::new() };
                let mut fallback_image = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut common_terms_and_conditions_items = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut confirmation_text = "";
                let mut error_text = "";
                let mut confirm_button_text = "";
                let mut file = File {
                    url: "",
                    details: FileDetails {
                        size: Number::from(0),
                        image: FileDetailsImage {
                            width: Number::from(0),
                            height: Number::from(0),
                        },
                    },
                    file_name: "",
                    content_type: "()",
                };
                let mut components = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut labels = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut configs = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut images = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });
                let mut target_view_port = "";
                let mut description = "";

                while let Some(key) = map.next_key()? {
                    match key {
                        "slug" => slug = map.next_value()?,
                        "title" => title = map.next_value()?,
                        "entryTitle" => entry_title = map.next_value()?,
                        "text" => text = map.next_value()?,
                        "link" => link = map.next_value()?,
                        "data" => data = map.next_value()?,
                        "fallbackImage" => fallback_image = map.next_value()?,
                        "commonTermsAndConditionsItems" => {
                            common_terms_and_conditions_items = map.next_value()?
                        }
                        "confirmationText" => confirmation_text = map.next_value()?,
                        "errorText" => error_text = map.next_value()?,
                        "confirmButtonText" => confirm_button_text = map.next_value()?,
                        "file" => file = map.next_value()?,
                        "components" => components = map.next_value()?,
                        "labels" => labels = map.next_value()?,
                        "configs" => configs = map.next_value()?,
                        "images" => images = map.next_value()?,
                        "targetViewport" => target_view_port = map.next_value()?,
                        "description" => description = map.next_value()?,
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        // _ => {
                        //     map.next_value()?;
                        // }
                    }
                }
                let slug = slug;
                let title = title;
                let entry_title = entry_title;
                let text = text;
                let link = link;
                let data = data;
                let fallback_image = fallback_image;
                let common_terms_and_conditions_items = common_terms_and_conditions_items;
                let confirmation_text = confirmation_text;
                let error_text = error_text;
                let confirm_button_text = confirm_button_text;
                let file = file;
                let components = components;
                let labels = labels;
                let configs = configs;
                let images = images;
                let target_view_port = target_view_port;
                let description = description;

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
