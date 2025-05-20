use serde::{Serialize, Deserialize};
use time::{Date, OffsetDateTime};
use utoipa::{schema, ToSchema, IntoParams};
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;

use crate::handlers::reviews::Review;
use crate::handlers::authors::Author;

#[derive(Serialize, ToSchema)]
pub struct Book {
    pub book_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub isbn_13: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub publisher: Option<String>,
    /* Time format: yyyy */
    pub published_date: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub categories: Option<Vec<String>>,
    pub rating_counts: i32,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
    pub cover_url: Option<String>,
    pub preview_link: Option<String>,
    pub info_link: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SearchQuery {
    pub title: Option<String>,
    pub isbn: Option<String>,
    pub author_name: Option<String>,
    pub rating: Option<i32>,
    pub publisher: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i64>,
}

#[utoipa::path(
    tag = "books",
    params(
        SearchQuery
    ),
    responses(
        (status = 200, description = "Successful operation.", body = [Book]),
        (status = 400, description = "Book not found.")
    ),
)]
#[get("/books")]
pub async fn get_books(query: web::Query<SearchQuery>) -> impl Responder {
    let body: Vec<Book> = vec![];
    HttpResponse::Ok().json(body)
}

#[utoipa::path(
    tag = "books",
    params(
        ("book_id" = i64, Path, description = "ID for the book")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Book),
        (status = 404, description = "Book not found.")
    )
)]
#[get("/books/{book_id}")]
pub async fn get_book_by_id(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_books)
        .service(get_book_by_id);
}
