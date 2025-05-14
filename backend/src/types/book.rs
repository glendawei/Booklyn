use serde::Serialize;
use crate::Review;

#[allow(non_snake_case)]
#[derive(Clone, Serialize)]
pub struct Book {
    title: String,
    id: u64,
    isbn: String,
    authorNames: Vec<String>,
    publishers: Vec<String>,
    publishYear: String,
    reviews: Vec<Review>,
    tags: Vec<String>,
}
