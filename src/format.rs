pub trait DisplayJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    // TODO
    // fn as_json(&self) -> Json<&Self> {
    //     Json(self)
    // }
}
