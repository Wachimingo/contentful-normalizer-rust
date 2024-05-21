use super::{common_structs::ChildSys, ItemEntry};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Item {
    Single(ChildSys),
    Multiple(Vec<ChildSys>),
}

pub enum ItemsFieldTypes {
    Slug(String),
    Title(String),
    Item(Item),
    Entry(ItemEntry),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ItemsFields {
    pub slug: String,
    pub title: Option<String>,
    pub labels: Option<Item>,
    pub configs: Option<Item>,
    pub images: Option<Item>,
}

impl IntoIterator for ItemsFields {
    type Item = (String, Option<ItemsFieldTypes>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("slug".to_string(), Some(ItemsFieldTypes::Slug(self.slug))),
            ("title".to_string(), self.title.map(ItemsFieldTypes::Title)),
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
