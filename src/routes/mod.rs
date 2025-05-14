use crate::{Book, Review, SearchQuery};
use actix_web::{get, post, put, delete, web, Responder, HttpResponse, Either};

macro_rules! not_implemented {
    () => { HttpResponse::Ok().body("The funtionality is not implemented yet.") };
}

#[post("/login")]
pub async fn login() -> impl Responder {
    not_implemented!()
}

#[post("/logout")]
pub async fn logout() -> impl Responder {
    not_implemented!()
}

#[get("/books")]
pub async fn get_books() -> impl Responder {
    not_implemented!()
}

#[get("/books/{book_id}")]
pub async fn get_book_by_id(id: web::Path<i64>) -> impl Responder {
    not_implemented!()
}

#[get("/search")]
pub async fn search_book_by_query(query: web::Query<SearchQuery>) -> impl Responder {
    not_implemented!()
}

#[post("/review")]
pub async fn create_review() -> impl Responder {
    not_implemented!()
}

#[put("/review")]
pub async fn update_review() -> impl Responder {
    not_implemented!()
}

#[delete("/delete/{review_id}")]
pub async fn delete_review_by_id(id: web::Path<i64>) -> impl Responder {
    not_implemented!()
}
