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
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUser {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
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
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[patch("/users/{user_id}")]
pub async fn update_user_by_id(data: web::Data<AppData>, id: web::Path<i64>, body: web::Json<UpdateUser>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    let mut tx = data.db_conn.begin().await?;
    
    if let Some(user) = sqlx::query_as!(
        User,
        r#"
        UPDATE "users" SET
            "display_name" = COALESCE($1, "display_name"),
            "avatar"       = COALESCE($2, "avatar"),
            "bio"          = COALESCE($3, "bio"),
            "website"      = COALESCE($4, "website")
        WHERE "user_id" = $5
        RETURNING "user_id", "display_name", "email", "role", "bio", "avatar", "website", "created_at";
        "#,
        body.display_name,
        body.avatar,
        body.bio,
        body.website,
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
