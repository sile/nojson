use nojson::{JsonParseError, RawJson};

#[test]
fn parse_jsonc_line_comments() -> Result<(), JsonParseError> {
    let text = r#"{
        "name": "John", // This is a line comment
        "age": 30 // Another comment
    }"#;

    let (json, comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify JSON parsing works correctly
    let name: String = json.value().to_required_member("name")?.try_into()?;
    assert_eq!(name, "John");

    let age: i32 = json.value().to_required_member("age")?.try_into()?;
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
    let name: String = json.value().to_required_member("name")?.try_into()?;
    assert_eq!(name, "Alice");

    let city: String = json.value().to_required_member("city")?.try_into()?;
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
    let users_array = json.value().to_required_member("users")?.to_array()?;
    let users: Vec<_> = users_array.collect();
    assert_eq!(users.len(), 2);

    let first_user_name: String = users[0].to_required_member("name")?.try_into()?;
    assert_eq!(first_user_name, "Bob");

    let second_user_name: String = users[1].to_required_member("name")?.try_into()?;
    assert_eq!(second_user_name, "Carol");

    // Verify all comments were detected
    assert_eq!(comment_ranges.len(), 7);

    Ok(())
}

#[test]
fn parse_jsonc_trailing_commas_object() -> Result<(), JsonParseError> {
    let text = r#"{
        "name": "John",
        "age": 30,
        "active": true,
    }"#;

    let (json, _comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify JSON parsing works correctly with trailing comma
    let name: String = json.value().to_required_member("name")?.try_into()?;
    assert_eq!(name, "John");

    let age: i32 = json.value().to_required_member("age")?.try_into()?;
    assert_eq!(age, 30);

    let active: bool = json.value().to_required_member("active")?.try_into()?;
    assert_eq!(active, true);

    Ok(())
}

#[test]
fn parse_jsonc_trailing_commas_array() -> Result<(), JsonParseError> {
    let text = r#"{
        "fruits": [
            "apple",
            "banana",
            "cherry",
        ],
        "numbers": [1, 2, 3,]
    }"#;

    let (json, _comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify array parsing with trailing commas
    let fruits_array = json.value().to_required_member("fruits")?.to_array()?;
    let fruits: Vec<String> = fruits_array
        .map(|item| item.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(fruits, vec!["apple", "banana", "cherry"]);

    let numbers_array = json.value().to_required_member("numbers")?.to_array()?;
    let numbers: Vec<i32> = numbers_array
        .map(|item| item.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(numbers, vec![1, 2, 3]);

    Ok(())
}

#[test]
fn parse_jsonc_trailing_commas_with_comments() -> Result<(), JsonParseError> {
    let text = r#"{
        "config": {
            "debug": true, // Enable debug mode
            "port": 8080, // Server port
        }, // End config
        "features": [
            "auth", // Authentication
            "logging", // Request logging
        ], // End features
    }"#;

    let (json, comment_ranges) = RawJson::parse_jsonc(text)?;

    // Verify parsing with both trailing commas and comments
    let debug: bool = json
        .value()
        .to_required_member("config")?
        .to_required_member("debug")?
        .try_into()?;
    assert_eq!(debug, true);

    let port: i32 = json
        .value()
        .to_required_member("config")?
        .to_required_member("port")?
        .try_into()?;
    assert_eq!(port, 8080);

    let features_array = json.value().to_required_member("features")?.to_array()?;
    let features: Vec<String> = features_array
        .map(|item| item.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(features, vec!["auth", "logging"]);

    // Verify comments were detected
    assert_eq!(comment_ranges.len(), 6);

    Ok(())
}

#[test]
fn parse_jsonc_abnormal_trailing_commas() {
    // Test cases for abnormal trailing commas that should all fail to parse
    let test_cases = vec![
        // Empty containers with commas
        ("[,]", "Empty array with comma"),
        ("{,}", "Empty object with comma"),
        // Double commas
        ("[1,,]", "Array with double comma at end"),
        (r#"{"a": 1,,}"#, "Object with double comma at end"),
        ("[1,2,,3]", "Array with double comma in middle"),
        (r#"{"a":1,,"b":2}"#, "Object with double comma in middle"),
        // Leading commas
        ("[,1,2]", "Array with leading comma"),
        (r#"{,"a": 1}"#, "Object with leading comma"),
        // Multiple consecutive commas
        ("[1,,,2]", "Array with multiple consecutive commas"),
        ("[,,,]", "Array with only commas"),
        ("{,,}", "Object with only commas"),
        // Abnormal commas with comments
        (
            r#"[
            1, // First element
            , // This comma has no element before it
            2
        ]"#,
            "Array with abnormal comma placement with comments",
        ),
        // Nested structures with abnormal commas
        (
            r#"{
            "outer": {
                "inner": [1,,2]
            }
        }"#,
            "Nested structure with abnormal comma",
        ),
        // Additional edge cases
        ("[1,,]", "Double comma at end"),
        ("[,,1]", "Multiple leading commas"),
        (r#"{"a":,}"#, "Object with comma after key-value separator"),
        ("[1,2,3,,]", "Multiple trailing commas"),
    ];

    for (test_case, description) in test_cases {
        let result = RawJson::parse_jsonc(test_case);
        assert!(
            result.is_err(),
            "{} should fail to parse. Input: '{}'",
            description,
            test_case
        );
    }
}
