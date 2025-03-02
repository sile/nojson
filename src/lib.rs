pub trait WriteJson {
    // TOD: type Error;
    fn write_json<T: ToJsonWriter>(&mut self, value: &T) -> std::io::Result<()>;
}

pub trait ReadJson {
    // TOD: type Error;
    fn read_json<T: FromJsonReader>(&mut self) -> std::io::Result<T>;
}

pub trait ToJsonWriter {
    fn to_json_writer<W: WriteJson>(&self, writer: W) -> std::io::Result<()>;
}

pub trait FromJsonReader: Sized {
    fn from_json_reader<R: ReadJson>(reader: R) -> std::io::Result<Self>;
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
