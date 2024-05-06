use rust_contentful_normalizer::normalize_helpers::{normalize_labels, ContentfulEntity, ContentfulIncludes, Entries, Fields, Sys};




fn main() {
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
    let result = normalize_labels(labels, includes);
    println!("{:#?}", result)
}
