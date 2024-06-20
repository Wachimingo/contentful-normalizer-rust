use super::common_structs::ChildSys;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item {
    Single(ChildSys),
    Multiple(Vec<ChildSys>),
}
pub enum ItemRef<'a> {
    Single(&'a ChildSys),
    Multiple(&'a Vec<ChildSys>),
}

pub trait SingleMultiple<'a, U, V> {
    type SingleType;
    type MultipleType;
    fn single(&'a self) -> Option<U>;
    fn multiple(&'a self) -> Option<V>;
}

impl<'a> SingleMultiple<'a, ChildSys, &'a Vec<ChildSys>> for Item {
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

impl<'a> SingleMultiple<'a, &'a ChildSys, Vec<& 'a ChildSys>> for Item {
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

impl<'a> SingleMultiple<'a, &'a ChildSys, &'a Vec<ChildSys>> for ItemRef<'a> {
    type SingleType = &'a ChildSys;
    type MultipleType = &'a Vec<ChildSys>;

    fn single(&self) -> Option<Self::SingleType> {
        match self {
            Self::Single(single) => Some(single),
            _ => None,
        }
    }
    fn multiple(&self) -> Option<Self::MultipleType> {
        match self {
            Self::Multiple(multiple) => Some(multiple),
            _ => None,
        }
    }
}

struct ItemVisitor;

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item;

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

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ItemVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemsFieldTypes {
    Text(String),
    Item(Item),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemsFields {
    pub slug: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub seo: Option<serde_json::Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Item>,
}

impl IntoIterator for ItemsFields {
    type Item = (String, Option<ItemsFieldTypes>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("slug".to_string(), Some(ItemsFieldTypes::Text(self.slug))),
            ("title".to_string(), self.title.map(ItemsFieldTypes::Text)),
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
