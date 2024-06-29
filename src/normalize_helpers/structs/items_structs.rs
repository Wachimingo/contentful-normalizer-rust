use crate::normalize_helpers::structs::common_structs::ChildSysInner;

use super::common_structs::ChildSys;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item<T: Clone + Copy> {
    Single(T),
    Multiple(Vec<T>),
}

struct ItemVisitor;

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item<ChildSys<'de>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Not a valid item")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }
        Ok(Item::Multiple(vec))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(Item::Single(Deserialize::deserialize(
            serde::de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

impl<'de> Deserialize<'de> for Item<ChildSys<'de>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ItemVisitor)
    }
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum ItemsFieldTypes<'a> {
//     Text(&'a str),
//     Item(Item<ChildSys<'a>>),
// }

#[derive(Clone, Debug, Serialize)]
pub struct ItemsFields<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub components: Item<ChildSys<'a>>,
    pub labels: Item<ChildSys<'a>>,
    pub configs: Item<ChildSys<'a>>,
    pub images: Item<ChildSys<'a>>,
    pub visitor_state: Item<ChildSys<'a>>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ItemsFields<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemsFieldsVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ItemsFieldsVisitor<'a> {
            type Value = ItemsFields<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ItemsFields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ItemsFields<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut slug = "";
                let mut title = "";
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
                let mut visitor_state = Item::Single(ChildSys {
                    sys: ChildSysInner {
                        id: "",
                        link_type: "",
                        object_type: "",
                    },
                });

                while let Some(key) = map.next_key()? {
                    match key {
                        "slug" => {
                            slug = map.next_value()?;
                        }
                        "title" => {
                            title = map.next_value()?;
                        }
                        "components" => {
                            components = map.next_value()?;
                        }
                        "labels" => {
                            labels = map.next_value()?;
                        }
                        "configs" => {
                            configs = map.next_value()?;
                        }
                        "images" => {
                            images = map.next_value()?;
                        }
                        "visitorState" => {
                            visitor_state = map.next_value()?;
                        }
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }
                let slug = slug;
                let title = title;
                let components = components;
                let labels = labels;
                let configs = configs;
                let images = images;
                let visitor_state = visitor_state;

                Ok(ItemsFields {
                    slug,
                    title,
                    components,
                    labels,
                    configs,
                    images,
                    visitor_state,
                })
            }
        }

        const FIELDS: &'static [&'static str] =
            &["slug", "title", "components", "labels", "configs", "images"];
        deserializer.deserialize_struct(
            "ItemsFields",
            FIELDS,
            ItemsFieldsVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

// impl<'a> IntoIterator for ItemsFields<'a> {
//     type Item = (String, Option<ItemsFieldTypes<'a>>);
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         vec![
//             ("slug".to_string(), Some(ItemsFieldTypes::Text(self.slug))),
//             ("title".to_string(), self.title.map(ItemsFieldTypes::Text)),
//             (
//                 "components".to_string(),
//                 self.components.map(ItemsFieldTypes::Item),
//             ),
//             ("labels".to_string(), self.labels.map(ItemsFieldTypes::Item)),
//             (
//                 "configs".to_string(),
//                 self.configs.map(ItemsFieldTypes::Item),
//             ),
//             ("images".to_string(), self.images.map(ItemsFieldTypes::Item)),
//         ]
//         .into_iter()
//     }
// }
