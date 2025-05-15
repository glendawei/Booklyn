use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub title: Option<String>,
    pub isbn: Option<String>,
    pub author_name: Option<String>,
    pub rating: Option<u8>,
    pub publisher: Option<String>,
}
