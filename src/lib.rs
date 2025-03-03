pub trait WriteJson {
    type Error;

    fn write_json_null(&mut self) -> Result<(), Self::Error>;
    fn write_json_bool(&mut self, value: bool) -> Result<(), Self::Error>;
    fn write_json_i64(&mut self, value: i64) -> Result<(), Self::Error>;
    fn write_json_f64(&mut self, value: f64) -> Result<(), Self::Error>;
    fn write_json_str(&mut self, value: &str) -> Result<(), Self::Error>;
    fn write_json_array<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(&mut Self);
    fn write_json_array_value<T: ToJson>(&mut self, value: &T) -> Result<(), Self::Error>;
    fn write_json_object<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(&mut Self);
    fn write_json_object_member<T: ToJson>(
        &mut self,
        name: &str,
        value: &T,
    ) -> Result<(), Self::Error>;
}

pub trait ReadJson {
    type Error;
    fn read_json<T: FromJson>(&mut self) -> Result<T, Self::Error>;
}

pub trait ToJson {
    fn to_json<W: WriteJson>(&self, writer: W) -> Result<(), W::Error>;
}

pub trait FromJson: Sized {
    fn from_json<R: ReadJson>(reader: R) -> Result<Self, R::Error>;
}

impl ToJson for i64 {
    fn to_json<W: WriteJson>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_json_i64(*self)
    }
}

impl ToJson for String {
    fn to_json<W: WriteJson>(&self, mut writer: W) -> Result<(), W::Error> {
        writer.write_json_str(self)
    }
}

#[derive(Debug)]
pub struct IoJsonWriter {}

#[derive(Debug)]
pub struct IoJsonReader<R> {
    pub inner: R,
    pub line: usize,
    pub column: usize,
    pub path: Vec<()>, // TODO
}

#[derive(Debug)]
pub struct StrJsonReader<'a> {
    pub json: &'a str,
    pub line: usize,
    pub column: usize,
    pub path: Vec<()>, // TODO
}

// JsonlJsonReader
// DiagnosticJsonReader
