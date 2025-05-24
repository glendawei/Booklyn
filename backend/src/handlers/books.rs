use serde::{Serialize, Deserialize};
use time::{Date, OffsetDateTime};
use utoipa::{ToSchema, IntoParams};
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;

use crate::handlers::reviews::Review;
use crate::handlers::authors::Author;
use crate::AppData;
use crate::database;
use crate::error::Error;

/* default maximum number of returned books */
const LIMIT: i64 = 50;

#[derive(Serialize, ToSchema, Default)]
pub struct Book {
    pub book_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub isbn_13: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub publisher: Option<String>,
    #[schema(value_type = String)]
    pub published_date: Option<Date>,
    pub reviews: Option<Vec<Review>>,
    pub categories: Option<Vec<String>>,
    pub ratings_count: Option<i32>,
    #[schema(value_type = String)]
    pub created_at: Option<OffsetDateTime>,
    pub cover_url: Option<String>,
    pub preview_link: Option<String>,
    pub info_link: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SearchQuery {
    pub title: Option<String>,
    pub isbn: Option<String>,
    pub author_name: Option<String>,
    pub publisher: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i64>,
}

/* TODO: Current implementation is considered inefficient. Redesign the whole procedure to improve performance. */
#[utoipa::path(
    tag = "books",
    params(
        SearchQuery
    ),
    responses(
        (status = 200, description = "Successful operation.", body = [Book]),
        (status = 404, description = "Books not found."),
        (status = 500, description = "Internal server error", body = String)
    ),
)]
#[get("/books")]
pub async fn get_books(data: web::Data<AppData>, query: web::Query<SearchQuery>) -> Result<impl Responder, Error> {
    let query = query.into_inner();
    let limit = query.limit.unwrap_or(LIMIT);

    let book_ids = sqlx::query!(
        r#"
        SELECT DISTINCT "b"."book_id"
        FROM "books" AS "b"
            JOIN "book_authors" AS "ba" ON "b"."book_id" = "ba"."book_id"
            JOIN "authors" AS "a" ON "ba"."author_id" = "a"."author_id"
        WHERE
            CASE WHEN $2::text IS NOT NULL THEN "b"."title" LIKE CONCAT('%', $2, '%')
                 ELSE true
            END AND
            CASE WHEN $3::text IS NOT NULL THEN "b"."isbn_13" = $3
                 ELSE true
            END AND
            CASE WHEN $4::text IS NOT NULL THEN "a"."name" LIKE CONCAT('%', $4, '%')
                 ELSE true
            END AND
            CASE WHEN $5::text IS NOT NULL THEN "b"."publisher" = $5
                 ELSE true
            END AND
            CASE WHEN $6::text IS NOT NULL THEN "b"."categories" @> '{$6}'
                 ELSE true
            END
        LIMIT $1;
        "#,
        limit,
        query.title,
        query.isbn,
        query.author_name,
        query.publisher,
        query.category
    )
        .fetch_all(&data.db_conn)
        .await?;

    if book_ids.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut books = Vec::new();

    for id in book_ids {
        if let Some(book) = database::get_book_by_id(&data.db_conn, id.book_id).await? {
            books.push(book);
        }
    }

    if books.len() > 0 { Ok(HttpResponse::Ok().json(books)) } else { Ok(HttpResponse::NotFound().finish()) }
}

#[utoipa::path(
    tag = "books",
    params(
        ("book_id" = i64, Path, description = "ID for the book")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Book),
        (status = 404, description = "Book not found."),
        (status = 500, description =  "Internal server error", body = String)
    )
)]
#[get("/books/{book_id}")]
pub async fn get_book_by_id(data: web::Data<AppData>, id: web::Path<i64>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    
    if let Some(book) = database::get_book_by_id(&data.db_conn, id).await? {
        Ok(HttpResponse::Ok().json(book))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_books)
        .service(get_book_by_id);
}
