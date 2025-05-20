use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Comment {
    pub comment_id: i64,
    pub review_id: Option<i64>,
    pub user_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub content: String,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateComment {
    pub review_id: i64,
    pub user_id: i64,
    pub parent_id: i64,
    pub content: String,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateComment {
    pub content: String,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 400, description = "Invalid request. Maybe the timestamp format is wrong.")
    )
)]
#[get("/reviews/{review_id}/comments/{comment_id}")]
pub async fn get_comment_by_id(ids: web::Path<(i64, i64)>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 400, description = "Invalid reqeust. Maybe the timestamp format is wrong.")
    )
)]
#[post("/reviews/{review_id}/comments")]
pub async fn create_comment(id: web::Path<i64>, body: web::Json<CreateComment>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 400, description = "Invalid request. Maybe the timestamp format is wrong."),
        (status = 404, description = "Comment not found.")
    )
)]
#[patch("/reviews/{review_id}/comments/{comment_id}")]
pub async fn update_comment_by_id(ids: web::Path<(i64, i64)>, body: web::Json<UpdateComment>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation."),
        (status = 404, description = "Comment not found.")
    )
)]
#[delete("/reviews/{review_id}/comments/{comment_id}")]
pub async fn delete_comment_by_id(ids: web::Path<(i64, i64)>) -> impl Responder {
    HttpResponse::Ok()
}
