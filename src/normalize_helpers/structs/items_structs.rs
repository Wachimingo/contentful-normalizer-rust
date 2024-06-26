use super::common_structs::ChildSys;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub trait SingleMultiple<'a, U, V> {
    type SingleType;
    type MultipleType;
    fn single(&'a self) -> Option<U>;
    fn multiple(&'a self) -> Option<V>;
}

impl<'a> SingleMultiple<'a, ChildSys, &'a Vec<ChildSys>> for Item<ChildSys> {
    type SingleType = ChildSys;
    type MultipleType = Vec<ChildSys>;

    fn single(&self) -> Option<Self::SingleType> {
        match self {            
            Self::Single(single) => Some(single.clone()),
            _ => None,
        }
    }
    fn multiple(&'a self) -> Option<&Self::MultipleType> {
        match self {
            Self::Multiple(multiple) => Some(multiple),
            _ => None,
        }
    }
}

impl<'a> SingleMultiple<'a, &'a ChildSys, Vec<& 'a ChildSys>> for Item<ChildSys> {
    type SingleType = &'a ChildSys;
    type MultipleType = Vec<& 'a ChildSys>;

    fn single(&'a self) -> Option<Self::SingleType> {
        match self {            
            Self::Single(single) => Some(single),
            _ => None,
        }
    }
    fn multiple(&'a self) -> Option<Self::MultipleType> {
        match self {
            Self::Multiple(multiple) => {
                let mut new_vec: Vec<&ChildSys> = Vec::new();
                for child_sys in multiple {
                    new_vec.push(child_sys)
                }
                Some(new_vec)
            },
            _ => None,
        }
    }
}

struct ItemVisitor;

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item<ChildSys>;

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

impl<'de> Deserialize<'de> for Item<ChildSys> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ItemVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemsFieldTypes<'a> {
    Text(String),
    Text2(&'a str),
    Item(Item<ChildSys>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemsFields<'a> {
    pub slug: &'a str,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub seo: Option<serde_json::Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Item<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Item<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Item<ChildSys>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Item<ChildSys>>,
}

impl<'a> IntoIterator for ItemsFields<'a> {
    type Item = (String, Option<ItemsFieldTypes<'a>>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("slug".to_string(), Some(ItemsFieldTypes::Text2(self.slug))),
            ("title".to_string(), self.title.map(ItemsFieldTypes::Text2)),
            (
                "components".to_string(),
                self.components.map(ItemsFieldTypes::Item),
            ),
            ("labels".to_string(), self.labels.map(ItemsFieldTypes::Item)),
            (
                "configs".to_string(),
                self.configs.map(ItemsFieldTypes::Item),
            ),
            ("images".to_string(), self.images.map(ItemsFieldTypes::Item)),
        ]
        .into_iter()
    }
}
