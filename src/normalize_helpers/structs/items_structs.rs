use serde::{Deserialize, Serialize};

use super::common_structs::ChildSys;

pub type Labels = Vec<ChildSys>;
pub type Configs = Vec<ChildSys>;
pub type Images = Vec<ChildSys>;

// #[derive(Clone, Deserialize, Serialize)]
// pub enum Labels {
//     Single(ChildSys),
//     Multiple(Vec<ChildSys>)
// }

// #[derive(Clone, Deserialize, Serialize)]
// pub enum Configs {
//     Single(ChildSys),
//     Multiple(Vec<ChildSys>)
// }
// #[derive(Clone, Deserialize, Serialize)]
// pub enum Images {
//     Single(ChildSys),
//     Multiple(Vec<ChildSys>)
// }

pub enum ItemsFieldTypes {
    Slug(String),
    Title(String),    
    Labels(Labels),
    Configs(Configs),
    Images(Images),    
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ItemsFields {
    pub slug: String,
    pub title: Option<String>,
    pub labels: Option<Labels>,
    pub configs: Option<Configs>,
    pub images: Option<Images>
}

impl IntoIterator for ItemsFields {
    type Item = (String, Option<ItemsFieldTypes>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("slug".to_string(), Some(ItemsFieldTypes::Slug(self.slug))),
            ("title".to_string(), self.title.map(ItemsFieldTypes::Title)),
            ("labels".to_string(), self.labels.map(ItemsFieldTypes::Labels)),
            ("configs".to_string(), self.configs.map(ItemsFieldTypes::Configs)),
            ("images".to_string(), self.images.map(ItemsFieldTypes::Images)),
        ].into_iter()
    }
}
