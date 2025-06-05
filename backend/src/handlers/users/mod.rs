mod reading_list;

pub use reading_list::*;

use actix_web::{get, patch, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

use crate::database;
use crate::AppData;
use crate::error::Error;

#[derive(Serialize, ToSchema)]
pub struct User {
    pub user_id: i64,
    pub display_name: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
    pub preffered_topics: Option<Vec<String>>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUser {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
    pub preffered_topics: Option<Vec<String>>,
}

pub fn check_preffered_topics(topics: &Vec<String>) -> bool {
    topics.len() > 0 && topics.len() <= 5
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, description = "ID for the user")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = User),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[get("/users/{user_id}")]
pub async fn get_user_by_id(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    if let Some(user) = database::get_user_by_id(&data.db_conn, id.into_inner()).await? {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, description = "ID for the user")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Successful operation.", body = User),
        (status = 400, description = "Bad reqeust. The amount of the preffered topics is invalid.", body = String),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[patch("/users/{user_id}")]
pub async fn update_user_by_id(data: web::Data<AppData>, id: web::Path<i64>, body: web::Json<UpdateUser>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    let mut tx = data.db_conn.begin().await?;

    if body.preffered_topics.is_some() && !check_preffered_topics(body.preffered_topics.as_ref().unwrap()) {
        return Ok(HttpResponse::BadRequest().content_type("text/plain; charset=utf-8").body("The amount of the preffered topics is restricted in [1, 5]."));
    }
    
    if let Some(user) = sqlx::query_as!(
        User,
        r#"
        UPDATE "users" SET
            "display_name"     = COALESCE($1, "display_name"),
            "avatar"           = COALESCE($2, "avatar"),
            "bio"              = COALESCE($3, "bio"),
            "website"          = COALESCE($4, "website"),
            "preffered_topics" = COALESCE($5, "preffered_topics")
        WHERE "user_id" = $6
        RETURNING "user_id", "display_name", "email", "role", "bio", "avatar", "website", "created_at", "preffered_topics";
        "#,
        body.display_name,
        body.avatar,
        body.bio,
        body.website,
        body.preffered_topics.as_deref(),
        id
    )
        .fetch_optional(&mut *tx)
        .await? 
    {
        tx.commit().await?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_user_by_id)
        .service(update_user_by_id)
        .service(get_reading_list_items)
        .service(create_reading_list_item)
        .service(update_reading_list_item_by_id)
        .service(delete_reading_list_item_by_id);
}
