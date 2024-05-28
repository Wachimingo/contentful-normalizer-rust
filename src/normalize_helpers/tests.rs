// use super::*;
// // use std::collections::HashMap;

// // #[test]
// // fn it_normalized_labels_succesfuly() {
    
// //     let mut expect_result = HashMap::new();
// //     expect_result.insert("exampleSlug1".to_string(), "Example Text 1".to_string());
// //     expect_result.insert("exampleSlug2".to_string(), "Example Text 2".to_string());

// //     assert_eq!(normalize_labels(labels, includes), expect_result)
// // }

// #[test]
// fn test_normalize_configs() {
//     let configs_str = r#"
//         [
//               {
//                 "sys": {
//                   "id": "1"
//                 }
//               },
//               {
//                 "sys": {
//                   "id": "2"
//                 }
//               }
//             ]
//         "#;
//     let includes_str = r#"
//         {
//             "entries": [
//               {
//                 "sys": {
//                   "id": "1",
//                   "type": "Type1",
//                   "createdAt": "2022-01-01T00:00:00Z",
//                   "updatedAt": "2022-01-01T00:00:00Z",
//                   "locale": "en-US"
//                 },
//                 "fields": {
//                   "slug": "sample-slug",
//                   "text": "Sample Text",
//                   "data": [
//                     {
//                         "key1": "value1",
//                         "key2": "value2"
//                     },
//                     {
//                         "key3": "value3",
//                         "key4": "value4"
//                     }
//                 ]
//                 }
//               },
//               {
//                 "sys": {
//                   "id": "2",
//                   "type": "Type2",
//                   "createdAt": "2022-01-01T00:00:00Z",
//                   "updatedAt": "2022-01-01T00:00:00Z",
//                   "locale": "en-US"
//                 },
//                 "fields": {
//                   "slug": "sample-slug-2",
//                   "text": "Sample Text 2",
//                   "data": [
//                     {
//                         "key5": "value5",
//                         "key6": "value6"
//                     },
//                     {
//                         "key7": "value7",
//                         "key8": "value8"
//                     }
//                 ]
//                 }
//               }
//             ]
//           }"#;
//     let configs: Vec<ContentfulEntity> =
//         serde_json::from_str(configs_str).expect("Error in congis parsing");
//     let includes: ContentfulIncludes =
//         serde_json::from_str(includes_str).expect("Error parsing includes");
//     // let result = normalize_configs(configs, includes);
//     normalize_configs(configs, includes);
//     // let result_json = serde_json::to_string(&result).expect("error parsing to string");
//     // let expected_result_json = r#"{"sampleSlug2":[{"key6":"value6","key5":"value5"},{"key7":"value7","key8":"value8"}],"sampleSlug":[{"key1":"value1","key2":"value2"},{"key4":"value4","key3":"value3"}]}"#;
// }
