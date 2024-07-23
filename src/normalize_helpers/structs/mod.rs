use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
pub mod common_structs;
pub mod includes_structs;
pub mod items_structs;
pub mod result_structs;
use std::fmt;

use self::{
    common_structs::TopLevelSys, includes_structs::IncludesFields, items_structs::ItemsFields,
};

#[derive(Clone, Debug, Serialize)]
pub struct IncludesEntry<'a> {
    pub sys: TopLevelSys<'a>,
    pub fields: IncludesFields<'a>,
}

impl<'de> Deserialize<'de> for IncludesEntry<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncludesEntryVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de> Visitor<'de> for IncludesEntryVisitor<'de> {
            type Value = IncludesEntry<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IncludesEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<IncludesEntry<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sys = None;
                let mut fields = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sys" => {
                            if sys.is_some() {
                                return Err(serde::de::Error::duplicate_field("sys"));
                            }
                            sys = Some(map.next_value()?);
                        }
                        "fields" => {
                            if fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let sys = sys.ok_or_else(|| serde::de::Error::missing_field("sys"))?;
                let fields = fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?;

                Ok(IncludesEntry { sys, fields })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys", "fields"];
        deserializer.deserialize_struct(
            "IncludesEntry",
            FIELDS,
            IncludesEntryVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemEntry<'a> {
    pub sys: TopLevelSys<'a>,
    pub fields: ItemsFields<'a>,
}

impl<'de> Deserialize<'de> for ItemEntry<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemEntryVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de> Visitor<'de> for ItemEntryVisitor<'de> {
            type Value = ItemEntry<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ItemEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ItemEntry<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sys = None;
                let mut fields = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sys" => {
                            if sys.is_some() {
                                return Err(serde::de::Error::duplicate_field("sys"));
                            }
                            sys = Some(map.next_value()?);
                        }
                        "fields" => {
                            if fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let sys = sys.ok_or_else(|| serde::de::Error::missing_field("sys"))?;
                let fields = fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?;

                Ok(ItemEntry { sys, fields })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys", "fields"];
        deserializer.deserialize_struct(
            "ItemEntry",
            FIELDS,
            ItemEntryVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulIncludes<'a> {
    #[serde(rename = "Entry")]
    pub entries: Vec<IncludesEntry<'a>>,
    #[serde(rename = "Asset")]
    pub assets: Vec<IncludesEntry<'a>>,
}

impl<'de> Deserialize<'de> for ContentfulIncludes<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulIncludesVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de> Visitor<'de> for ContentfulIncludesVisitor<'de> {
            type Value = ContentfulIncludes<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulIncludes")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulIncludes<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut entries = None;
                let mut assets = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "Entry" => {
                            if entries.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries = Some(map.next_value()?);
                        }
                        "Asset" => {
                            if assets.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown assets
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let entries = entries.ok_or_else(|| serde::de::Error::missing_field("entries"))?;
                let assets = assets.ok_or_else(|| serde::de::Error::missing_field("assets"))?;

                Ok(ContentfulIncludes { entries, assets })
            }
        }

        const FIELDS: &'static [&'static str] = &["entries", "assets"];
        deserializer.deserialize_struct(
            "ContentfulIncludes",
            FIELDS,
            ContentfulIncludesVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContentfulResponse<'a> {
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
                let mut total = None;
                let mut skip = None;
                let mut limit = None;
                let mut items = None;
                let mut includes = None;

                while let Some(key) = map.next_key()? {
                    match key {                    
                        "total" => {
                            if total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total = Some(map.next_value()?);
                        }
                        "skip" => {
                            if skip.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip"));
                            }
                            skip = Some(map.next_value()?);
                        }
                        "limit" => {
                            if limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit = Some(map.next_value()?);
                        }
                        "items" => {
                            if items.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items = Some(map.next_value()?);
                        }
                        "includes" => {
                            if includes.is_some() {
                                return Err(serde::de::Error::duplicate_field("includes"));
                            }
                            includes = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown assets
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                // let sys =
                //     sys.ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_sys"))?;
                let total =
                    total.ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_total"))?;
                let skip =
                    skip.ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_skip"))?;
                let limit =
                    limit.ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_limit"))?;
                let items =
                    items.ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_items"))?;
                let includes = includes
                    .ok_or_else(|| serde::de::Error::missing_field("ContentfulResponse_includes"))?;

                Ok(ContentfulResponse {
                    // sys,
                    total,
                    skip,
                    limit,
                    items,
                    includes,
                })
            }
        }

        const FIELDS: &'static [&'static str] =
            &["sys", "total", "skip", "limit", "items", "includes"];
        deserializer.deserialize_struct(
            "ContentfulResponse",
            FIELDS,
            ContentfulResponseVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}