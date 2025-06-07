use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

use crate::database;
use crate::AppData;
use crate::error::Error;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Author {
    pub author_id: i64,
    pub user_id: Option<i64>,
    pub name: String,
    pub bio: Option<String>,
    pub website: Option<String>,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}

#[utoipa::path(
    tag = "authors",
    params(
        ("author_id" = i64, Path, description = "ID for the author")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Author),
        (status = 404, description = "Author not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[get("/authors/{author_id}")]
pub async fn get_author_by_id(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    if let Some(author) = database::get_author_by_id(&data.db_conn, id.into_inner()).await? {
        Ok(HttpResponse::Ok().json(author))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_author_by_id);
}
