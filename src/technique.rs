use url::Url;

#[derive(Debug)]
pub struct Technique {
    profile: String,
    xmlns: Option<Url>,
    content: String
}
