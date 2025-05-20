mod reading_list;

pub use reading_list::*;

use actix_web::{get, patch, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use utoipa::{schema, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct User {
    pub user_id: i64,
    pub display_name: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
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
        (status = 404, description = "User not found.")
    )
)]
#[get("/users/{user_id}")]
pub async fn get_users_by_id(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, description = "ID for the user")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Successful operation.", body = User),
        (status = 404, description = "User not found")
    )
)]
#[patch("/users/{user_id}")]
pub async fn update_user_by_id(id: web::Path<i64>, body: web::Json<UpdateUser>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_users_by_id)
        .service(update_user_by_id)
        .service(get_reading_list_items)
        .service(create_reading_list_item)
        .service(update_reading_list_item_by_id)
        .service(delete_reading_list_item_by_id);
}
