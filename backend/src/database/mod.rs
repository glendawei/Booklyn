use sqlx::{
    error::Error,
    postgres::PgPool
};
use time::{Date, OffsetDateTime};
use serde::Serialize;

use crate::handlers::{
    books::Book,
    authors::Author,
    users::User,
    reviews::Review
};

#[derive(Serialize)]
struct BookRecord {
    pub book_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub isbn_13: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<Date>,
    pub categories: Option<Vec<String>>,
    pub ratings_count: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
    pub cover_url: Option<String>,
    pub preview_link: Option<String>,
    pub info_link: Option<String>,
}

impl BookRecord {
    pub fn into_book(self) -> Book {
        let mut book = Book::default();

        book.book_id = self.book_id;
        book.title = self.title;
        book.description = self.description;
        book.isbn_13 = self.isbn_13;
        book.publisher = self.publisher;
        book.published_date = self.published_date;
        book.categories = self.categories;
        book.ratings_count = self.ratings_count;
        book.created_at = self.created_at;
        book.cover_url = self.cover_url;
        book.preview_link = self.preview_link;
        book.info_link = self.info_link;
        book
    }
}

pub async fn get_book_by_id(db_conn: &PgPool, id: i64) -> Result<Option<Book>, Error> {
    let mut tx = db_conn.begin().await?;

    if let Some(book_record) = sqlx::query_as!(
        BookRecord,
        r#"
        SELECT
            "book_id",
            "title",
            "description",
            "isbn_13",
            "publisher",
            "published_date",
            "categories",
            "ratings_count",
            "created_at",
            "cover_url",
            "preview_link",
            "info_link"
        FROM "books"
        WHERE "book_id" = $1;
        "#,
        id
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        let authors = sqlx::query_as!(
            Author,
            r#"
            SELECT "a"."author_id", "user_id", "name", "bio", "website", "created_at"
            FROM "book_authors" AS "ba"
                JOIN "authors" AS "a" ON "ba"."author_id" = "a"."author_id"
            WHERE "ba"."book_id" = $1;
            "#,
            id
        )
            .fetch_all(&mut *tx)
            .await?;

        let reviews = sqlx::query_as!(
            Review,
            r#"
            SELECT 
            "r"."review_id",
            "r"."book_id",
            "user_id",
            "is_external",
            "source",
            "source_review_id",
            "user_id_src",
            "profile_name",
            "rating",
            "review_time",
            "summary",
            "content",
            "helpful_yes" AS "upvotes",
            "helpful_total" - "helpful_yes" AS "downvotes",
            "ra"."credibility_score"
            FROM "reviews" as "r"
                LEFT JOIN "review_ai" AS "ra" ON "r"."review_id" = "ra"."review_id"
                JOIN "books" AS "b" ON "r"."book_id" = "b"."book_id"
            WHERE "r"."book_id" = $1;
            "#,
            id
        )
            .fetch_all(&mut *tx)
            .await?;
 
        let mut book = book_record.into_book();

        book.authors = if authors.len() > 0 { Some(authors) } else { None };
        book.reviews = if reviews.len() > 0 { Some(reviews) } else { None };
        Ok(Some(book))
    } else {
        Ok(None)
    }
}

pub async fn get_author_by_id(db_conn: &PgPool, id: i64) -> Result<Option<Author>, Error> {
    sqlx::query_as!(
        Author,
        r#"
        SELECT "author_id", "user_id", "name", "bio", "website", "created_at"
        FROM "authors"
        WHERE "author_id" = $1;
        "#,
        id
    )
        .fetch_optional(db_conn)
        .await
}

pub async fn get_user_by_id(db_conn: &PgPool, id: i64) -> Result<Option<User>, Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT "user_id", "email", "display_name", "role", "bio", "avatar", "website", "created_at"
        FROM "users"
        WHERE "user_id" = $1;
        "#,
        id
    )
        .fetch_optional(db_conn)
        .await
}

pub async fn get_review_by_id(db_conn: &PgPool, id: i64) -> Result<Option<Review>, Error> {
    sqlx::query_as!(
        Review,
        r#"
        SELECT 
            "r"."review_id",
            "book_id",
            "user_id",
            "is_external",
            "source",
            "source_review_id",
            "user_id_src",
            "profile_name",
            "rating",
            "review_time",
            "summary",
            "content",
            "helpful_yes" AS "upvotes",
            "helpful_total" - "helpful_yes" AS "downvotes",
            "ra"."credibility_score"
        FROM "reviews" as "r"
            LEFT JOIN "review_ai" AS "ra" ON "r"."review_id" = "ra"."review_id"
        WHERE "r"."review_id" = $1;
        "#,
        id
    )
        .fetch_optional(db_conn)
        .await
}
