pub trait DisplayJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    // TODO
    // fn as_json(&self) -> Json<&Self> {
    //     Json(self)
    // }
}

impl DisplayJson for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        let mut chars = self.chars();
        while let Some(c) = chars.next() {
            match c {
                '\n' => write!(f, r#"\n"#)?,
                '\r' => write!(f, r#"\r"#)?,
                '\t' => write!(f, r#"\t"#)?,
                '\\' => write!(f, r#"\\"#)?,
                '\"' => write!(f, r#"\""#)?,
                '\x08' => write!(f, r#"\b"#)?,
                '\x0C' => write!(f, r#"\f"#)?,
                c if c.is_control() => write!(f, r#"\u{:04x}"#, c as u32)?,
                _ => write!(f, "{c}")?,
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

// TODO: impls
