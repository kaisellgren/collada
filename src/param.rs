use asset::Asset;

#[derive(Debug)]
pub struct Param {
    sid: Option<String>,
    name: Option<String>,
    typ: String,
    semantic: Option<Asset>
}
