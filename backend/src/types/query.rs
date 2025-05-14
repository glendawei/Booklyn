use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    title: Option<String>,
    isbn: Option<String>,
    author_name: Option<String>,
    rating: Option<u8>,
    publisher: Option<String>,
}
