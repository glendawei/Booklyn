use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use utoipa::{schema, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct ReadingListItem {
    pub item_id: i64,
    pub book_id: i64,
    pub status: String,
    pub note: Option<String>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateReadingListItem {
    pub book_id: i64,
    pub status: String,
    pub note: Option<String>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateReadingListItem {
    pub status: Option<String>,
    pub note: Option<String>,
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = [ReadingListItem]),
        (status = 404, description = "User not found.")
    )
)]
#[get("/users/{user_id}/reading-list")]
pub async fn get_reading_list_items(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user")
    ),
    request_body = CreateReadingListItem,
    responses(
        (status = 200, description = "Successful operation.", body = ReadingListItem),
        (status = 400, description = "Invalid request. Maybe the timestamp format is wrong."),
        (status = 404, description = "User not found.")
    )
)]
#[post("/users/{user_id}/reading-list")]
pub async fn create_reading_list_item(id: web::Path<i64>, body: web::Json<CreateReadingListItem>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user"),
        ("item_id" = i64, Path, description = "ID for the reading list item")
    ),
    request_body = UpdateReadingListItem,
    responses(
        (status = 200, description = "Successful operation.", body = ReadingListItem),
        (status = 400, description = "Invalid request. Maybe the timestamp format is wrong."),
        (status = 404, description = "User not found or the reading list item not found.")
    )
)]
#[patch("/users/{user_id}/reading-list/{item_id}")]
pub async fn update_reading_list_item_by_id(ids: web::Path<(i64, i64)>, body: web::Json<UpdateReadingListItem>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user"),
        ("item_id" = i64, Path, description = "ID for the reading list item")
    ),
    responses(
        (status = 200, description = "Successful operation."),
        (status = 404, description = "Reading list item not found.")
    )
)]
#[delete("/users/{user_id}/reading-list/{item_id}")]
pub async fn delete_reading_list_item_by_id(ids: web::Path<(i64, i64)>) -> impl Responder {
    HttpResponse::Ok()
}
