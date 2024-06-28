use super::common_structs::ChildSys;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item<T:Clone+Copy> {
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
    pub slug: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Item<ChildSys<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Item<ChildSys<'a>>>,
    #[serde(rename = "visitorState",skip_serializing_if = "Option::is_none")]
    pub visitor_state: Option<Item<ChildSys<'a>>>,
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
                let mut slug = None;
                let mut title = None;
                let mut components = None;
                let mut labels = None;
                let mut configs = None;
                let mut images = None;
                let mut visitor_state = None;

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
                        },
                        "visitorState" => {
                            if visitor_state.is_some() {
                                return Err(de::Error::duplicate_field("visitorState"));
                            }
                            visitor_state = Some(map.next_value()?);
                        }
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }
                let slug = slug.unwrap_or_else(|| None);
                let title = title.unwrap_or_else(|| None);
                let components =
                    components.unwrap_or_else(|| None);
                let labels = labels.unwrap_or_else(|| None);
                let configs = configs.unwrap_or_else(|| None);
                let images = images.unwrap_or_else(|| None);
                let visitor_state = visitor_state.unwrap_or_else(|| None);

                Ok(ItemsFields {
                    slug,
                    title,
                    components,
                    labels,
                    configs,
                    images,
                    visitor_state
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
