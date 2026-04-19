---
name: nojson
description: >
  Work with the nojson Rust crate: parse and generate JSON via the `Json<T>`
  wrapper, `RawJson`/`RawJsonOwned`/`RawJsonValue` for low-level access, the
  `DisplayJson` trait, and in-place builders `json()` / `object()` / `array()`.
  Implement `DisplayJson` + `TryFrom<RawJsonValue<'_, '_>>` to round-trip custom
  types; navigate objects with `to_member` / `to_path_member` and
  `required()` / `optional()`; produce rich error context via
  `RawJsonValue::invalid()` and `JsonParseError::get_line_and_column_numbers`;
  handle JSONC with `parse_jsonc`. Use when the task mentions nojson, its
  no-macro/no-dependency JSON API, or no_std JSON work with this crate.
license: MIT
compatibility: Requires Rust 1.88+ and cargo.
metadata:
  author: sile
  version: "0.3.10"
---

# nojson

Use this skill when integrating `nojson` into Rust code. Focus on the crate's
actual API shape and usage patterns, not generic JSON background.

## What this crate exposes

- `Json<T>` — wrapper giving `FromStr` / `Display` for any `T` that implements
  the corresponding nojson traits
- `DisplayJson` — JSON analogue of `core::fmt::Display`
- `RawJson<'text>` / `RawJsonOwned` — a parsed, validated JSON text that has
  not been converted to Rust types yet
- `RawJsonValue<'text, 'raw>` — a node inside a `RawJson`; the entry point for
  reading values
- `RawJsonMember<'text, 'raw, 'a>` — result of `to_member`; resolves with
  `required()` / `optional()` / `TryInto<Option<T>>`
- `JsonFormatter` / `JsonArrayFormatter` / `JsonObjectFormatter` — passed into
  `DisplayJson::fmt` and the in-place builder closures
- Free functions `json()`, `object()`, `array()` — return values that
  `impl Display + DisplayJson`
- `JsonValueKind`, `JsonParseError`

## Choosing the right API

1. **Typed round-trip** — use `Json<T>` with
   `T: DisplayJson + for<'t, 'r> TryFrom<RawJsonValue<'t, 'r>, Error = JsonParseError>`.
   Parse via `text.parse::<Json<T>>()`, format via `Json(&value).to_string()`.
2. **Inline output (incl. pretty-printing)** — use `json()`, `object()`, or
   `array()` with a closure. Good for one-off logging / CLI output without
   defining a struct.
3. **Imperative navigation / validation** — `RawJson::parse(text)?`, then
   traverse with `RawJsonValue` methods. Use this when you need positions,
   conditional fields, or rich error context.
4. **Owned JSON detached from the input lifetime** — use `RawJsonOwned` or
   `RawJsonValue::extract().into_owned()`.
5. **JSON with comments / trailing commas** — `RawJson::parse_jsonc` /
   `RawJsonOwned::parse_jsonc`. Returns `(RawJson, Vec<Range<usize>>)` where
   the ranges are comment byte spans in the original text.

## Usage gotchas

- The crate is `#![no_std]` + `alloc`. The default `std` feature enables
  `DisplayJson` / `TryFrom` impls for `HashMap`, `HashSet`, `PathBuf`,
  `IpAddr`, `SocketAddr`, etc. Turn it off for `no_std` builds.
- Custom parsing implements `TryFrom<RawJsonValue<'text, 'raw>>` with
  `Error = JsonParseError` — not `FromStr`. `FromStr` on `Json<T>` delegates
  to this `TryFrom` impl.
- `to_member(name)` does a linear scan. If you read many siblings, prefer
  `to_object()` once and match keys yourself.
- `to_path_member(&["a","b","c"])` is a convenience. Intermediate keys are
  required; only the last may be handled as optional.
- `as_string_str()` returns `Err` when the JSON string has escapes; it never
  allocates. For general decoding use `to_unquoted_string_str()` — it returns
  `Cow<'text, str>` (borrowed when no escapes).
- `f32` / `f64` that are not finite (NaN, ±Infinity) serialize as `null`. JSON
  has no NaN literal; do not expect round-trips through floats.
- `()` serializes to / deserializes from `null`. `Option<T>::None` also maps to
  `null`.
- Pretty-printing is controlled on the formatter:
  `f.set_indent_size(n)` + `f.set_spacing(true)`. Settings apply to the current
  depth and deeper; inner `nojson::json(|f| …)` can override locally.
- `RawJsonValue` is `Copy`; most traversal methods consume `self` by value on
  purpose — pass it around freely.
- `RawJsonValue::index()` is stable within one `RawJson`. Cache it and re-fetch
  with `get_value_by_index` for O(1) access after validation.

## API patterns to preserve

Typed round-trip for a custom struct:

```rust
struct Person { name: String, age: u32 }

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
    fn try_from(v: nojson::RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        Ok(Person {
            name: v.to_member("name")?.required()?.try_into()?,
            age:  v.to_member("age")?.required()?.try_into()?,
        })
    }
}

let p: nojson::Json<Person> = r#"{"name":"a","age":1}"#.parse()?;
let s = nojson::Json(&p.0).to_string();
```

Inline pretty-printed output:

```rust
let pretty = nojson::json(|f| {
    f.set_indent_size(2);
    f.set_spacing(true);
    f.object(|f| {
        f.member("items", &[1, 2, 3])?;
        f.member("enabled", true)
    })
});
let text = pretty.to_string();
```

Optional members:

```rust
let obj = json.value();
let city: Option<String> = obj.to_member("city")?.try_into()?; // None if absent
let age: Option<u32> = obj.to_member("age")?.map(|v| v.try_into())?;
```

Validation with precise error context:

```rust
let json = nojson::RawJson::parse(text)?;
let v = json.value();
let n: u32 = v.as_number_str()?.parse().map_err(|e| v.invalid(e))?;
if n == 0 {
    return Err(v.invalid("must be positive"));
}
```

Error reporting with line / column:

```rust
if let Err(err) = nojson::RawJson::parse(text) {
    if let Some((line, col)) = err.get_line_and_column_numbers(text) {
        eprintln!("{err} at {line}:{col}");
    }
    if let Some(line) = err.get_line(text) {
        eprintln!("  {line}");
    }
}
```

JSONC:

```rust
let (json, comments) = nojson::RawJson::parse_jsonc(text)?;
// comments: Vec<Range<usize>> over the original text — use for tooling,
// re-emission, or preserving documentation.
```

Sub-tree extraction:

```rust
let user = json.value().to_member("user")?.required()?;
let sub: nojson::RawJson<'_> = user.extract();       // borrowed
let owned = sub.into_owned();                        // RawJsonOwned
```

## Practical hints

- Reach for `Json<T>` first for straightforward typed parsing; drop to
  `RawJson` only when you need position info, conditional fields, or custom
  validation.
- Nested generics usually work via the blanket impls already in the crate:
  `Vec<T>`, `[T; N]`, `Option<T>`, `BTreeMap<K, V>`, `HashMap<K, V>`,
  `Rc<T>`, `Arc<T>`, etc. You rarely need to hand-write these.
- Map keys parse via `K: FromStr`. The JSON key is always a string, so
  `BTreeMap<u32, _>` is valid when the keys are numeric strings.
- For deeply nested single-field lookups use `to_path_member`. For multiple
  siblings, resolve the parent once and call `to_member` for each.
- The root value of a `RawJson` is always index 0 (`json.value()` ==
  `json.get_value_by_index(0).unwrap()`).
- `RawJsonOwned::object(|f| …)` / `::array(|f| …)` / `::json(|f| …)` are
  convenience constructors — they build, serialize, and re-parse; use them
  when you want an owned JSON value without going through `to_string`.
