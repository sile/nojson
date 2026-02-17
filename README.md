nojson
======

[![nojson](https://img.shields.io/crates/v/nojson.svg)](https://crates.io/crates/nojson)
[![Documentation](https://docs.rs/nojson/badge.svg)](https://docs.rs/nojson)
[![Actions Status](https://github.com/sile/nojson/workflows/CI/badge.svg)](https://github.com/sile/nojson/actions)
![License](https://img.shields.io/crates/l/nojson)

A flexible Rust JSON library with no dependencies and no macros.

`nojson` is a flexible and ergonomic JSON library for Rust that offers a balance between the type-safety of Rust and the dynamic nature of JSON.
Unlike [`serde`](https://crates.io/crates/serde), which typically requires one-to-one mapping between Rust types and JSON structures (or other serialization formats),
`nojson` provides a toolbox approach that allows you to leverage both type-level programming and imperative code flexibility.

## Features

- **No strict one-to-one type mapping required** - Mix type-level programming with imperative flexibility as needed
- **Clean parsing error messages** with position information for better debugging
- **Customizable validation** - Add application-specific validation rules with proper error context
- **Flexible formatting options** including pretty-printing with customizable indentation
- **Low-level access** to the JSON structure when needed
- **High-level conveniences** for common JSON operations

## Core Design Principles

- A toolbox rather than a monolithic framework
- Gain the benefits of both type-level programming and imperative code
- Easy to add custom validations with rich error context
- Error messages that precisely indicate the problematic position in the JSON text

## Getting Started

### Parsing JSON with Strong Typing

The `Json<T>` wrapper allows parsing JSON text into Rust types that implement `TryFrom<RawJsonValue<'_, '_>>`:

```rust
fn main() -> Result<(), nojson::JsonParseError> {
    // Parse a JSON array into a typed Rust array
    let text = "[1, null, 2]";
    let value: nojson::Json<[Option<u32>; 3]> = text.parse()?;
    assert_eq!(value.0, [Some(1), None, Some(2)]);
    Ok(())
}
```

### Generating JSON

The `DisplayJson` trait allows converting Rust types to JSON:

```rust
// Generate a JSON array from a Rust array
let value = [Some(1), None, Some(2)];
assert_eq!(nojson::Json(value).to_string(), "[1,null,2]");
```

### In-place JSON Generation with Formatting

The `json()` function provides a convenient way to generate JSON with custom formatting:

```rust
// Compact JSON
let compact = nojson::json(|f| f.value([1, 2, 3]));
assert_eq!(compact.to_string(), "[1,2,3]");

// Pretty-printed JSON with custom indentation
let pretty = nojson::json(|f| {
    f.set_indent_size(2);
    f.set_spacing(true);
    f.array(|f| {
        f.element(1)?;
        f.element(2)?;
        f.element(3)
    })
});

assert_eq!(
    format!("\n{}", pretty),
    r#"
[
  1,
  2,
  3
]"#
);
```

### Custom Types

Implementing `DisplayJson` and `TryFrom<RawJsonValue<'_, '_>>` for your own types:

```rust
struct Person {
    name: String,
    age: u32,
}

impl nojson::DisplayJson for Person {
    fn fmt(&self, f: &mut nojson::JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.object(|f| {
            f.member("name", &self.name)?;
            f.member("age", self.age)
        })
    }
}

impl<'text, 'raw> TryFrom<nojson::RawJsonValue<'text, 'raw>> for Person {
    type Error = nojson::JsonParseError;

    fn try_from(value: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let name = value.to_required_member("name")?;
        let age = value.to_required_member("age")?;
        Ok(Person {
            name: name.try_into()?,
            age: age.try_into()?,
        })
    }
}

fn main() -> Result<(), nojson::JsonParseError> {
    // Parse JSON to Person
    let json_text = r#"{"name":"Alice","age":30}"#;
    let person: nojson::Json<Person> = json_text.parse()?;

    // Generate JSON from Person
    assert_eq!(nojson::Json(&person.0).to_string(), json_text);

    Ok(())
}
```

## Advanced Features

### Custom Validations

You can add custom validations using `RawJsonValue::invalid()`:

```rust
fn parse_positive_number(text: &str) -> Result<u32, nojson::JsonParseError> {
    let json = nojson::RawJson::parse(text)?;
    let raw_value = json.value();

    let num: u32 = raw_value.as_number_str()?
        .parse()
        .map_err(|e| raw_value.invalid(e))?;

    if num == 0 {
        return Err(raw_value.invalid("Expected a positive number, got 0"));
    }

    Ok(num)
}
```

### Error Handling with Context

Rich error information helps with debugging:

```rust
let text = r#"{"invalid": 123e++}"#;
let result = nojson::RawJson::parse(text);

if let Err(error) = result {
    println!("Error: {}", error);

    // Get line and column information
    if let Some((line, column)) = error.get_line_and_column_numbers(text) {
        println!("At line {}, column {}", line, column);
    }

    // Get the full line with the error
    if let Some(line_text) = error.get_line(text) {
        println!("Line content: {}", line_text);
    }
}
```
