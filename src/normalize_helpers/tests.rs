use std::collections::HashMap;

use crate::normalize_helpers::{ContentfulEntity, ContentfulIncludes, Entries, Fields, Sys};

use super::normalize_labels;

#[test]
fn it_normalized_labels_succesfuly() {
    let labels = vec![
        ContentfulEntity {
            sys: Sys {
                id: String::from("1"),
                // ... other fields
            },
            // ... other fields
        },
        ContentfulEntity {
            sys: Sys {
                id: String::from("2"),
                // ... other fields
            },
            // ... other fields
        },
    ];

    let includes = ContentfulIncludes {
        entries: vec![
            Entries {
                sys: Sys {
                    id: String::from("1"),
                    // ... other fields
                },
                fields: Fields {
                    slug: String::from("example-slug-1"),
                    text: String::from("Example Text 1"),
                },
                // ... other fields
            },
            Entries {
                sys: Sys {
                    id: String::from("2"),
                    // ... other fields
                },
                fields: Fields {
                    slug: String::from("example-slug-2"),
                    text: String::from("Example Text 2"),
                },
                // ... other fields
            },
        ],
        // ... other fields
    };

    let mut expect_result = HashMap::new();
    expect_result.insert("exampleSlug1".to_string(), "Example Text 1".to_string());
    expect_result.insert("exampleSlug2".to_string(), "Example Text 2".to_string());

    assert_eq!(normalize_labels(labels, includes), expect_result)
}
