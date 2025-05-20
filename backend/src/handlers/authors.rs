use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Serialize;
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Author {
    pub author_id: i64,
    pub user_id: Option<i64>,
    pub name: String,
    pub bio: Option<String>,
    pub website: Option<String>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[utoipa::path(
    tag = "authors",
    params(
        ("author_id" = i64, Path, description = "ID for the author")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Author),
        (status = 404, description = "Author not found.")
    )
)]
#[get("/authors/{author_id}")]
pub async fn get_authors_by_id(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_authors_by_id);
}
