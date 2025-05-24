use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

use crate::AppData;
use crate::error::Error;

#[derive(Serialize, ToSchema)]
pub struct ReadingListItem {
    pub item_id: i64,
    pub user_id: Option<i64>,
    pub book_id: Option<i64>,
    pub status: String,
    pub note: Option<String>,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateReadingListItem {
    pub book_id: i64,
    pub status: String,
    pub note: Option<String>,
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
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/users/{user_id}/reading-list")]
pub async fn get_reading_list_items(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    let mut tx = data.db_conn.begin().await?;

    if sqlx::query!(
        r#"SELECT user_id FROM users WHERE user_id = $1;"#,
        id
    )
        .fetch_optional(&mut *tx)
        .await?.is_none()
    {
        return Ok(HttpResponse::NotFound().finish())
    }
    
    let items = sqlx::query_as!(
        ReadingListItem,
        r#"SELECT * FROM reading_list_items WHERE user_id = $1;"#,
        id
    )
        .fetch_all(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(HttpResponse::Ok().json(items))
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user")
    ),
    request_body = CreateReadingListItem,
    responses(
        (status = 200, description = "Successful operation.", body = ReadingListItem),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/users/{user_id}/reading-list")]
pub async fn create_reading_list_item(
    data: web::Data<AppData>,
    id: web::Path<i64>,
    body: web::Json<CreateReadingListItem>
) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let create_item = body.into_inner();
    let mut tx = data.db_conn.begin().await?;
    
    match sqlx::query_as!(
        ReadingListItem,
        r#"
        INSERT INTO reading_list_items (user_id, book_id, status, note, created_at)
        SELECT $1, $2, $3, $4, NOW()
        WHERE EXISTS (
            SELECT *
            FROM users
            WHERE user_id = $1
        )
        RETURNING item_id, user_id, book_id, status, note, created_at;
        "#,
        user_id,
        create_item.book_id,
        create_item.status,
        create_item.note,
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(item) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(item))
        },
        None => Ok(HttpResponse::NotFound().finish())
    }
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
        (status = 404, description = "User not found or the reading list item not found."),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[patch("/users/{user_id}/reading-list/{item_id}")]
pub async fn update_reading_list_item_by_id(
    data: web::Data<AppData>,
    ids: web::Path<(i64, i64)>,
    body: web::Json<UpdateReadingListItem>
) -> Result<impl Responder, Error> {
    let (_, item_id) = ids.into_inner();
    let update = body.into_inner();
    let mut tx = data.db_conn.begin().await?;

    match sqlx::query_as!(
        ReadingListItem,
        r#"
        UPDATE "reading_list_items" SET
            "status" = COALESCE($1, "status"),
            "note"   = COALESCE($2, "note")
        WHERE "item_id" = $3
        RETURNING "item_id", "user_id", "book_id", "status", "note", "created_at";
        "#,
        update.status,
        update.note,
        item_id
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(item) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(item))
        },
        None => Ok(HttpResponse::NotFound().finish())
    }
}

#[utoipa::path(
    tag = "users",
    params(
        ("user_id" = i64, Path, description = "ID for the user"),
        ("item_id" = i64, Path, description = "ID for the reading list item")
    ),
    responses(
        (status = 200, description = "Successful operation."),
        (status = 404, description = "Reading list item not found."),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[delete("/users/{user_id}/reading-list/{item_id}")]
pub async fn delete_reading_list_item_by_id(
    data: web::Data<AppData>,
    ids: web::Path<(i64, i64)>
) -> Result<impl Responder , Error> {
    let (_, item_id) = ids.into_inner();
    let mut tx = data.db_conn.begin().await?;
    
    let result = sqlx::query!(
        r#"
        DELETE FROM "reading_list_items"
        WHERE "item_id" = $1;
        "#,
        item_id
    )
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 { Ok(HttpResponse::NotFound().finish()) } else { Ok(HttpResponse::Ok().finish()) }
}
