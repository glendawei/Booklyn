use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Vote {
    pub user_id: i64,
    pub review_id: i64,
    pub helpful: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateVote {
    pub user_id: i64,
    pub helpful: bool,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Vote),
    )
)]
#[post("/reviews/{review_id}/vote")]
pub async fn create_vote(id: web::Path<i64>, body: web::Json<CreateVote>) -> impl Responder {
    HttpResponse::Ok()
}
