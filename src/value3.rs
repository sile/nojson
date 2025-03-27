pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];
pub const NUMBER_PREFIX: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
pub struct Error {}

#[derive(Debug)]
pub enum Kind {
    Null,
    Bool,
    Integer,
    Float,
    String,
    StringEscaped,
    Array,
    Object,
}

// TODO: rename
#[derive(Debug)]
pub struct JsonValue {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct JsonParser<'a> {
    pub text: &'a str,
    pub index: usize,
    pub values: Vec<JsonValue>,
}

impl<'a> JsonParser<'a> {
    pub fn parse(&mut self) -> Result<(), Error> {
        self.strip_whitespaces();

        if self.text.starts_with("null") {
            self.push_value(Kind::Null, "null".len());
        } else if self.text.starts_with("true") {
            self.push_value(Kind::Bool, "true".len());
        } else if self.text.starts_with("false") {
            self.push_value(Kind::Bool, "false".len());
        } else if self.text.starts_with(NUMBER_PREFIX) {
            self.parse_number()?;
        } else if self.text.starts_with('"') {
            todo!()
        } else if self.text.starts_with('[') {
            todo!()
        } else if self.text.starts_with('{') {
            todo!()
        }
        Ok(())
    }

    fn parse_number(&mut self) -> Result<(), Error> {
        let s = self.text.strip_prefix('-').unwrap_or(self.text);
        let s = s.strip_prefix(DIGITS).expect("TODO");
        let s = s.trim_start_matches(DIGITS);

        let (kind, s) = if let Some(s) = s.strip_prefix('.') {
            let s = s.strip_prefix(DIGITS).expect("TODO");
            let s = s.trim_start_matches(DIGITS);
            (Kind::Float, s)
        } else {
            (Kind::Integer, s)
        };

        let n = self.text.len() - s.len();
        self.push_value(kind, n);

        Ok(())
    }

    fn push_value(&mut self, kind: Kind, len: usize) {
        let start = self.index;
        let end = start + len;
        self.values.push(JsonValue { kind, start, end });
        self.index = end;
        self.text = &self.text[len..];
    }

    fn strip_whitespaces(&mut self) {
        let s = self.text.trim_start_matches(WHITESPACES);
        self.index += self.text.len() - s.len();
        self.text = s;
    }
}

#[derive(Debug)]
pub struct JsonText<'a> {
    pub text: &'a str,
    pub values: Vec<JsonValue>,
}

impl<'a> JsonText<'a> {
    pub fn new(text: &'a str) -> Result<Self, Error> {
        let mut parser = JsonParser {
            text,
            index: 0,
            values: Vec::new(),
        };
        parser.parse()?;
        Ok(Self {
            text,
            values: parser.values,
        })
    }
}
