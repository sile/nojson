use std::fmt::Display;

use crate::DisplayJson;

// pub fn value_with<F>(f:F) ->
// pub fn array_with()
// pub fn object_with()
// Json(json_with(|f| ...)).to_string()

pub struct JsonValue;

impl JsonValue {
    // fn null(self) -> impl DisplayJson {
    //     todo!()
    // }

    // pub fn maybe_finite_f64()
    // pub fn string()

    // pub fn array<F>(self, f: F) -> impl DisplayJson
    // where
    //     F: FnOnce(&mut JsonArray),
    // {
    //     todo!()
    // }
}

pub struct JsonArray;

impl JsonArray {
    pub fn value<T>(&mut self, _value: T)
    where
        T: DisplayJson,
    {
        // Json(JsonValue::Array(|a| {
        //     a.value("foo");
        //     a.value(1);
        //     a.values(iter)
        // }))
        // .to_string();
        todo!()
    }

    // values
}

pub struct JsonFormatter<'a> {
    fmt: &'a mut std::fmt::Formatter<'a>,
}

impl<'a> JsonFormatter<'a> {
    pub fn new(fmt: &'a mut std::fmt::Formatter<'a>) -> Self {
        Self { fmt }
    }

    pub fn null(self) -> std::fmt::Result {
        write!(self.fmt, "null")
    }

    pub fn bool(self, v: bool) -> std::fmt::Result {
        write!(self.fmt, "{v}")
    }

    pub fn integer<T>(self, v: T) -> std::fmt::Result
    where
        // TODO: TryFrom
        i64: From<T>,
    {
        write!(self.fmt, "{}", i64::from(v))
    }

    pub fn float<T>(self, v: T) -> std::fmt::Result
    where
        f64: From<T>,
    {
        // TODO: check finite
        write!(self.fmt, "{}", f64::from(v))
    }

    pub fn string<T>(self, _v: T) -> std::fmt::Result
    where
        T: Display,
    {
        todo!()
    }

    pub fn value<T>(self, _v: T) -> std::fmt::Result
    where
        T: DisplayJson,
    {
        todo!()
    }

    // pub fn array(self) -> JsonArrayFormatter<'a> {
    //     let ok = write!(self.fmt, "[").is_ok();
    //     JsonArrayFormatter {
    //         inner: ok.then_some(self),
    //         first: true,
    //     }
    // }
}

// pub struct JsonArrayFormatter<'a> {
//     inner: Option<JsonFormatter<'a>>,
//     first: bool,
// }

// impl<'a> JsonArrayFormatter<'a> {
//     pub fn value<T>(&mut self, _v: T) -> std::fmt::Result
//     where
//         T: DisplayJson,
//     {
//         todo!()
//     }
// }
