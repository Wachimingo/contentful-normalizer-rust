use core::fmt;

use serde::{Deserialize, Deserializer, Serialize, de::{self, Visitor, MapAccess}};
use serde_json::Value;
pub mod common_structs;
pub mod includes_structs;
pub mod items_structs;
pub mod result_structs;

use self::{
    common_structs::TopLevelSys, includes_structs::IncludesFields, items_structs::ItemsFields,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludesEntry {
    pub sys: TopLevelSys,
    pub fields: IncludesFields,
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemEntry<'slug> {
    pub metadata: Value,
    pub sys: TopLevelSys,
    pub fields: ItemsFields<'slug>,
}

impl<'de: 'slug, 'slug> Deserialize<'de> for ItemEntry<'slug> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemEntryVisitor<'slug> {
            marker: std::marker::PhantomData<&'slug ()>,
        }

        impl<'de: 'slug, 'slug> Visitor<'de> for ItemEntryVisitor<'slug> {
            type Value = ItemEntry<'slug>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ItemEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ItemEntry<'slug>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut metadata = None;
                let mut sys = None;
                let mut fields = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "metadata" => {
                            if metadata.is_some() {
                                return Err(de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        },
                        "sys" => {
                            if sys.is_some() {
                                return Err(de::Error::duplicate_field("sys"));
                            }
                            sys = Some(map.next_value()?);
                        },
                        "fields" => {
                            if fields.is_some() {
                                return Err(de::Error::duplicate_field("fields"));
                            }
                            fields = Some(map.next_value()?);
                        },
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }

                let metadata = metadata.ok_or_else(|| de::Error::missing_field("metadata"))?;
                let sys = sys.ok_or_else(|| de::Error::missing_field("sys"))?;
                let fields = fields.ok_or_else(|| de::Error::missing_field("fields"))?;

                Ok(ItemEntry { metadata, sys, fields })
            }
        }

        const FIELDS: &'static [&'static str] = &["metadata","sys", "fields"];
        deserializer.deserialize_struct("ItemEntry", FIELDS, ItemEntryVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentfulIncludes {
    #[serde(rename = "Entry")]
    pub entries: Vec<IncludesEntry>,
    #[serde(rename = "Asset")]
    pub assets: Vec<IncludesEntry>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulItems<'slug> {
    pub entries: Vec<ItemEntry<'slug>>,
}

impl<'de: 'slug, 'slug> Deserialize<'de> for ContentfulItems<'slug> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulItemsVisitor<'slug> {
            marker: std::marker::PhantomData<&'slug ()>,
        }

        impl<'de: 'slug, 'slug> Visitor<'de> for ContentfulItemsVisitor<'slug> {
            type Value = ContentfulItems<'slug>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulItems")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulItems<'slug>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut entries = None;                

                while let Some(key) = map.next_key()? {
                    match key {
                        "entries" => {
                            if entries.is_some() {
                                return Err(de::Error::duplicate_field("entries"));
                            }
                            entries = Some(map.next_value()?);
                        },   
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }

                let entries = entries.ok_or_else(|| de::Error::missing_field("entries"))?;

                Ok(ContentfulItems { entries })
            }
        }

        const FIELDS: &'static [&'static str] = &["entries"];
        deserializer.deserialize_struct("ContentfulItems", FIELDS, ContentfulItemsVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulResponse<'slug> {
    pub sys: Value,
    pub total: u8,
    pub skip: u8,
    pub limit: u8,
    pub items: Vec<ItemEntry<'slug>>,
    pub includes: ContentfulIncludes,
}

impl<'de: 'slug, 'slug> Deserialize<'de> for ContentfulResponse<'slug> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulResponseVisitor<'slug> {
            marker: std::marker::PhantomData<&'slug ()>,
        }

        impl<'de: 'slug, 'slug> Visitor<'de> for ContentfulResponseVisitor<'slug> {
            type Value = ContentfulResponse<'slug>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulResponse<'slug>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut _sys= None;
                let mut total= None;
                let mut skip= None;
                let mut limit= None;
                let mut items= None;
                let mut includes = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sys" => {
                            if _sys.is_some() {
                                return Err(de::Error::duplicate_field("sys"));
                            }
                            _sys = Some(map.next_value()?);
                        },
                        "total" => {
                            if total.is_some() {
                                return Err(de::Error::duplicate_field("total"));
                            }
                            total = Some(map.next_value()?);
                        },
                        "skip" => {
                            if skip.is_some() {
                                return Err(de::Error::duplicate_field("skip"));
                            }
                            skip = Some(map.next_value()?);
                        },
                        "limit" => {
                            if limit.is_some() {
                                return Err(de::Error::duplicate_field("limit"));
                            }
                            limit = Some(map.next_value()?);
                        },
                        "items" => {
                            if items.is_some() {
                                return Err(de::Error::duplicate_field("items"));
                            }
                            items = Some(map.next_value()?);
                        },
                        "includes" => {
                            if includes.is_some() {
                                return Err(de::Error::duplicate_field("includes"));
                            }
                            includes = Some(map.next_value()?);
                        },                                                
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }
                let _sys = _sys.ok_or_else(|| de::Error::missing_field("sys"))?;
                let total = total.ok_or_else(|| de::Error::missing_field("total"))?;
                let skip = skip.ok_or_else(|| de::Error::missing_field("skip"))?;
                let limit = limit.ok_or_else(|| de::Error::missing_field("limit"))?;
                let items = items.ok_or_else(|| de::Error::missing_field("items"))?;
                let includes = includes.ok_or_else(|| de::Error::missing_field("includes"))?;

                Ok(ContentfulResponse { sys: _sys, total, skip , limit, items, includes })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys","total", "skip", "limit", "items", "includes"];
        deserializer.deserialize_struct("ContentfulResponse", FIELDS, ContentfulResponseVisitor { marker: std::marker::PhantomData })
    }
}