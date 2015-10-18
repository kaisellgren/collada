use url::Url;

#[derive(Debug)]
pub struct Contributor {
    author: Option<String>,
    author_email: Option<String>,
    author_website: Option<Url>,
    authoring_tool: Option<String>,
    comments: Option<String>,
    copyright: Option<String>,
    source_data: Option<Url>
}
