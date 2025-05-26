use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

use crate::AppData;
use crate::error::Error;

#[derive(Serialize, ToSchema)]
pub struct Comment {
    pub comment_id: i64,
    pub review_id: Option<i64>,
    pub user_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub content: String,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateComment {
    pub user_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateComment {
    pub content: String,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review")
    ),
    responses(
        (status = 200, description = "Successful operation", body = [Comment]),
        (status = 404, description = "Comments not found"),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[get("/reviews/{review_id}/comments")]
pub async fn get_comments(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    let comments = sqlx::query_as!(
        Comment,
        r#"
        SELECT "comment_id", "review_id", "user_id", "parent_id", "content", "created_at"
        FROM "comments"
        WHERE "review_id" = $1;
        "#,
        id.into_inner()
    )
        .fetch_all(&data.db_conn)
        .await?;

    if comments.len() == 0 { Ok(HttpResponse::NotFound().finish()) } else { Ok(HttpResponse::Ok().json(comments)) }
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 404, description = "Comment not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[get("/reviews/{review_id}/comments/{comment_id}")]
pub async fn get_comment_by_id(data: web::Data<AppData>, ids: web::Path<(i64, i64)>) -> Result<impl Responder, Error> {
    let (_, comment_id) = ids.into_inner();

    match sqlx::query_as!(
        Comment,
        r#"
        SELECT "comment_id", "review_id", "user_id", "parent_id", "content", "created_at"
        FROM "comments"
        WHERE "comment_id" = $1;
        "#,
        comment_id
    )
        .fetch_optional(&data.db_conn)
        .await?
    {
        Some(comment) => Ok(HttpResponse::Ok().json(comment)),
        None => Ok(HttpResponse::NotFound().finish()) 
    }
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 404, description = "Review not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[post("/reviews/{review_id}/comments")]
pub async fn create_comment(
    data: web::Data<AppData>,
    id: web::Path<i64>,
    body: web::Json<CreateComment>
) -> Result<impl Responder, Error> {
    let review_id = id.into_inner();
    let create_comment = body.into_inner();
    let mut tx = data.db_conn.begin().await?;

    match sqlx::query_as!(
        Comment,
        r#"
        INSERT INTO "comments" ("review_id", "user_id", "parent_id", "content", "created_at")
        SELECT $1, $2, $3, $4, NOW()
        WHERE EXISTS (
            SELECT "review_id"
            FROM "reviews"
            WHERE "review_id" = $1
        )
        RETURNING "comment_id", "review_id", "user_id", "parent_id", "content", "created_at";
        "#,
        review_id,
        create_comment.user_id,
        create_comment.parent_id,
        create_comment.content
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(comment) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(comment))
        },
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Comment),
        (status = 404, description = "Comment not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[patch("/reviews/{review_id}/comments/{comment_id}")]
pub async fn update_comment_by_id(
    data: web::Data<AppData>,
    ids: web::Path<(i64, i64)>,
    body: web::Json<UpdateComment>
) -> Result<impl Responder, Error> {
    let (_, comment_id) = ids.into_inner();
    let update_comment = body.into_inner();
    let mut tx = data.db_conn.begin().await?;

    match sqlx::query_as!(
        Comment,
        r#"
        UPDATE "comments" SET
            "content" = COALESCE($1, "content")
        WHERE "comment_id" = $2
        RETURNING "comment_id", "review_id", "user_id", "parent_id", "content", "created_at";
        "#,
        update_comment.content,
        comment_id,
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(comment) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(comment))
        },
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the related review"),
        ("comment_id" = i64, Path, description = "ID for the comment")
    ),
    responses(
        (status = 200, description = "Successful operation."),
        (status = 404, description = "Comment not found."),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[delete("/reviews/{review_id}/comments/{comment_id}")]
pub async fn delete_comment_by_id(data: web::Data<AppData>, ids: web::Path<(i64, i64)>) -> Result<impl Responder, Error> {
    let (_, comment_id) = ids.into_inner();
    let mut tx = data.db_conn.begin().await?;

    let result = sqlx::query!(
        r#"
        DELETE FROM "comments"
        WHERE "comment_id" = $1;
        "#,
        comment_id
    )
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    if result.rows_affected() == 0 { Ok(HttpResponse::NotFound().finish()) } else { Ok(HttpResponse::Ok().finish()) }
}
