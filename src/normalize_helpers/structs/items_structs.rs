use super::common_structs::ChildSys;
use serde::{Serialize, Deserialize, Deserializer, de::{Visitor, SeqAccess, MapAccess}};
use std::fmt;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item {
    Single(ChildSys),
    Multiple(Vec<ChildSys>),
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
        Ok(Item::Single(Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))
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
            ("components".to_string(), self.components.map(ItemsFieldTypes::Item)),
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
