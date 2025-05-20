mod comments;
mod vote;

pub use comments::*;
pub use vote::*;

use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Serialize;
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Review {
    pub review_id: i64,
    pub book_id: i64,
    pub user_id: Option<i64>,
    pub is_external: bool,
    pub source: String,
    pub source_review_id: Option<String>,
    pub user_id_src: Option<String>,
    pub profile_name: Option<String>,
    pub rating: Option<i16>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub review_time: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub upvotes: i32,
    pub downvotes: i32,
    pub credibility_score: Option<f64>,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Review),
        (status = 404, description = "Review not found.")
    )
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_review_by_id)
        .service(get_comment_by_id)
        .service(create_comment)
        .service(update_comment_by_id)
        .service(delete_comment_by_id)
        .service(create_vote);
}
