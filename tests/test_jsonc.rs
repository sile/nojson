use nojson::{JsonParseError, RawJson};

#[test]
fn parse_jsonc_line_comments() -> Result<(), JsonParseError> {
    let text = r#"{
        "name": "John", // This is a line comment
        "age": 30 // Another comment
    }"#;

    let (json, comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify JSON parsing works correctly
    let name: String = json.value().to_member("name")?.required()?.try_into()?;
    assert_eq!(name, "John");

    let age: i32 = json.value().to_member("age")?.required()?.try_into()?;
    assert_eq!(age, 30);

    // Verify comments were detected
    assert_eq!(comment_ranges.len(), 2);

    // Check comment content
    let first_comment = &text[comment_ranges[0].clone()];
    assert_eq!(first_comment, "// This is a line comment");

    let second_comment = &text[comment_ranges[1].clone()];
    assert_eq!(second_comment, "// Another comment");

    Ok(())
}

#[test]
fn parse_jsonc_block_comments() -> Result<(), JsonParseError> {
    let text = r#"{
        /* This is a
           multi-line block comment */
        "name": "Alice",
        "city": "New York" /* inline block comment */
    }"#;

    let (json, comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify JSON parsing works correctly
    let name: String = json.value().to_member("name")?.required()?.try_into()?;
    assert_eq!(name, "Alice");

    let city: String = json.value().to_member("city")?.required()?.try_into()?;
    assert_eq!(city, "New York");

    // Verify comments were detected
    assert_eq!(comment_ranges.len(), 2);

    // Check comment content
    let first_comment = &text[comment_ranges[0].clone()];
    assert!(first_comment.contains("multi-line block comment"));

    let second_comment = &text[comment_ranges[1].clone()];
    assert_eq!(second_comment, "/* inline block comment */");

    Ok(())
}

#[test]
fn parse_jsonc_mixed_comments() -> Result<(), JsonParseError> {
    let text = r#"{
        // Line comment at start
        // Multiple '//' comments
        "users": [ // Array with comments
            {
                "name": "Bob", /* inline block */
                "age": 25
            },
            /* Block comment between elements */
            {
                "name": "Carol",
                "age": 35 // Final line comment
            }
        ]
        /* Final block comment */
    }"#;

    let (json, comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify JSON structure
    let users_array = json.value().to_member("users")?.required()?.to_array()?;
    let users: Vec<_> = users_array.collect();
    assert_eq!(users.len(), 2);

    let first_user_name: String = users[0].to_member("name")?.required()?.try_into()?;
    assert_eq!(first_user_name, "Bob");

    let second_user_name: String = users[1].to_member("name")?.required()?.try_into()?;
    assert_eq!(second_user_name, "Carol");

    // Verify all comments were detected
    assert_eq!(comment_ranges.len(), 7);

    Ok(())
}
