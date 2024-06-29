use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct ChildSysInner<'a> {
    pub id: &'a str,
    #[serde(rename = "linkType")]
    pub link_type: &'a str,
    #[serde(rename = "type")]
    pub object_type: &'a str,
}
#[derive(Copy, Clone, Debug, Serialize)]
pub struct ChildSys<'a> {
    pub sys: ChildSysInner<'a>,
}

impl<'a> ChildSys<'a> {
    pub fn default() -> Self {
        ChildSys {
            sys: ChildSysInner {
                id: "",
                link_type: "",
                object_type: "",
            },
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for ChildSys<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChildSysVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ChildSysVisitor<'a> {
            type Value = ChildSys<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ChildSys")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ChildSys<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sys = ChildSysInner {
                    id: "",
                    link_type: "",
                    object_type: "",
                };

                while let Some(key) = map.next_key()? {
                    match key {
                        "sys" => {
                            sys = map.next_value()?;
                        }
                        _ => return Err(de::Error::unknown_field(key, FIELDS)),
                    }
                }

                // let sys = sys.ok_or_else(|| de::Error::missing_field("ChildSys_sys"))?;
                let sys = sys;

                Ok(ChildSys { sys })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys"];
        deserializer.deserialize_struct(
            "ChildSys",
            FIELDS,
            ChildSysVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopLevelSys<'a> {
    pub id: &'a str,
    pub space: ChildSys<'a>,
    #[serde(rename = "type")]
    pub object_type: &'a str,
    #[serde(rename = "createdAt")]
    pub created_at: &'a str,
    #[serde(rename = "updatedAt")]
    pub updated_at: &'a str,
    pub environment: ChildSys<'a>,
    pub revision: u8,
    #[serde(default = "ChildSys::default", rename = "contentType")]
    pub content_type: ChildSys<'a>,
    pub locale: &'a str,
}
#[derive(Clone, Debug, Serialize)]
pub struct ContentfulEntity<'a> {
    pub sys: TopLevelSys<'a>,
}

impl<'de: 'a, 'a> Deserialize<'de> for ContentfulEntity<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContentfulEntityVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> Visitor<'de> for ContentfulEntityVisitor<'a> {
            type Value = ContentfulEntity<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ContentfulEntity")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ContentfulEntity<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sys = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sys" => {
                            if sys.is_some() {
                                return Err(de::Error::duplicate_field("sys"));
                            }
                            sys = Some(map.next_value()?);
                        }
                        _ => {}
                    }
                }

                let sys = sys.ok_or_else(|| de::Error::missing_field("ContentfulEntity_sys"))?;

                Ok(ContentfulEntity { sys })
            }
        }

        const FIELDS: &'static [&'static str] = &["sys"];
        deserializer.deserialize_struct(
            "ContentfulEntity",
            FIELDS,
            ContentfulEntityVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    tags: Option<Vec<String>>,
}
