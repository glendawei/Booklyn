mod comments;
mod vote;

pub use comments::*;
pub use vote::*;

use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Serialize;
use time::OffsetDateTime;
use bigdecimal::BigDecimal;
use utoipa::{schema, ToSchema};

use crate::database;
use crate::AppData;
use crate::error::Error;

#[derive(Serialize, ToSchema)]
pub struct Review {
    pub review_id: i64,
    pub book_id: Option<i64>,
    pub user_id: Option<i64>,
    pub is_external: Option<bool>,
    pub source: Option<String>,
    pub source_review_id: Option<String>,
    pub user_id_src: Option<String>,
    pub profile_name: Option<String>,
    pub rating: Option<i16>,
    #[schema(value_type = String)]
    pub review_time: Option<OffsetDateTime>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    #[schema(value_type = String)]
    pub credibility_score: Option<BigDecimal>,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Review),
        (status = 404, description = "Review not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    match database::get_review_by_id(&data.db_conn, id.into_inner()).await? 
    {
        Some(review) => Ok(HttpResponse::Ok().json(review)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_review_by_id)
        .service(get_comment_by_id)
        .service(create_comment)
        .service(update_comment_by_id)
        .service(delete_comment_by_id)
        .service(create_vote);
}
