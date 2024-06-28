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

#[derive(Clone, Debug, Serialize)]
pub struct IncludesEntry<'a> {
    pub metadata: Value,
    pub sys: TopLevelSys<'a>,
    pub fields: IncludesFields<'a>,
}

impl<'de:'a,'a> Deserialize<'de> for IncludesEntry<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncludesEntryVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for IncludesEntryVisitor<'a> {
            type Value = IncludesEntry<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IncludesEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<IncludesEntry<'a>, V::Error>
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
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }

                let metadata = metadata.ok_or_else(|| de::Error::missing_field("IncludesEntry_metadata"))?;
                let sys = sys.ok_or_else(|| de::Error::missing_field("IncludesEntry_sys"))?;
                let fields = fields.ok_or_else(|| de::Error::missing_field("IncludesEntry_fields"))?;

                Ok(IncludesEntry { metadata, sys, fields })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys", "fields"];
        deserializer.deserialize_struct("IncludesEntry", FIELDS, IncludesEntryVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemEntry<'a> {
    pub metadata: Value,
    pub sys: TopLevelSys<'a>,
    pub fields: ItemsFields<'a>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ItemEntry<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemEntryVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ItemEntryVisitor<'a> {
            type Value = ItemEntry<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ItemEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ItemEntry<'a>, V::Error>
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
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }

                let metadata = metadata.ok_or_else(|| de::Error::missing_field("ItemEntry_metadata"))?;
                let sys = sys.ok_or_else(|| de::Error::missing_field("ItemEntry_sys"))?;
                let fields = fields.ok_or_else(|| de::Error::missing_field("ItemEntry_fields"))?;

                Ok(ItemEntry { metadata, sys, fields })
            }
        }

        const FIELDS: &'static [&'static str] = &["metadata","sys", "fields"];
        deserializer.deserialize_struct("ItemEntry", FIELDS, ItemEntryVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulIncludes<'a> {
    #[serde(rename = "Entry")]
    pub entries: Vec<IncludesEntry<'a>>,
    #[serde(rename = "Asset")]
    pub assets: Vec<IncludesEntry<'a>>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ContentfulIncludes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulIncludesVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ContentfulIncludesVisitor<'a> {
            type Value = ContentfulIncludes<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulIncludes")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulIncludes<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut entries = None;
                let mut assets = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "Entry" => {
                            if entries.is_some() {
                                return Err(de::Error::duplicate_field("Entry"));
                            }
                            entries = Some(map.next_value()?);
                        },
                        "Asset" => {
                            if assets.is_some() {
                                return Err(de::Error::duplicate_field("Asset"));
                            }
                            assets = Some(map.next_value()?);
                        },                       
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }

                let entries = entries.ok_or_else(|| de::Error::missing_field("ContentfulIncludes_entries"))?;
                let assets = assets.ok_or_else(|| de::Error::missing_field("ContentfulIncludes_assets"))?;

                Ok(ContentfulIncludes { entries, assets })
            }
        }

        const FIELDS: &'static [&'static str] = &["entries","assets"];
        deserializer.deserialize_struct("ContentfulIncludes", FIELDS, ContentfulIncludesVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulItems<'a> {
    pub entries: Vec<ItemEntry<'a>>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ContentfulItems<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulItemsVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ContentfulItemsVisitor<'a> {
            type Value = ContentfulItems<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulItems")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulItems<'a>, V::Error>
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
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }

                let entries = entries.ok_or_else(|| de::Error::missing_field("ContentfulItems_entries"))?;

                Ok(ContentfulItems { entries })
            }
        }

        const FIELDS: &'static [&'static str] = &["entries"];
        deserializer.deserialize_struct("ContentfulItems", FIELDS, ContentfulItemsVisitor { marker: std::marker::PhantomData })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulResponse<'a> {
    pub sys: Value,
    pub total: u8,
    pub skip: u8,
    pub limit: u8,
    pub items: Vec<ItemEntry<'a>>,
    pub includes: ContentfulIncludes<'a>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ContentfulResponse<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulResponseVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ContentfulResponseVisitor<'a> {
            type Value = ContentfulResponse<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulResponse<'a>, V::Error>
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
                        // _ => return Err(de::Error::unknown_field(key, FIELDS)),
                        _ => {}
                    }
                }
                let _sys = _sys.ok_or_else(|| de::Error::missing_field("ContentfulResponse_sys"))?;
                let total = total.ok_or_else(|| de::Error::missing_field("ContentfulResponse_total"))?;
                let skip = skip.ok_or_else(|| de::Error::missing_field("ContentfulResponse_skip"))?;
                let limit = limit.ok_or_else(|| de::Error::missing_field("ContentfulResponse_limit"))?;
                let items = items.ok_or_else(|| de::Error::missing_field("ContentfulResponse_items"))?;
                let includes = includes.ok_or_else(|| de::Error::missing_field("ContentfulResponse_includes"))?;

                Ok(ContentfulResponse { sys: _sys, total, skip , limit, items, includes })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys","total", "skip", "limit", "items", "includes"];
        deserializer.deserialize_struct("ContentfulResponse", FIELDS, ContentfulResponseVisitor { marker: std::marker::PhantomData })
    }
}