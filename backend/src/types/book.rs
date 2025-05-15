use serde::Serialize;
use crate::Review;

#[allow(non_snake_case)]
#[derive(Clone, Serialize)]
pub struct Book {
    pub title: String,
    pub id: u64,
    pub isbn: String,
    pub authorNames: Vec<String>,
    pub publishers: Vec<String>,
    pub publishYear: String,
    pub reviews: Vec<Review>,
    pub tags: Vec<String>,
}
