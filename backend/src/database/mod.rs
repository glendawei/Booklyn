use std::collections::HashMap;
use sqlx::{
    error::Error,
    postgres::PgPool,
    FromRow
};
use time::{Date, OffsetDateTime};

use crate::handlers::{
    books::Book,
    authors::Author,
    users::User,
    reviews::Review
};

#[derive(FromRow)]
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
    pub ai_summary: Option<String>,
}

impl BookRecord {
    pub fn into_book(self) -> Book {
        Book {
            book_id: self.book_id,
            title: self.title,
            description: self.description,
            isbn_13: self.isbn_13,
            publisher: self.publisher,
            published_date: self.published_date,
            categories: self.categories,
            ratings_count: self.ratings_count,
            created_at: self.created_at,
            cover_url: self.cover_url,
            preview_link: self.preview_link,
            info_link: self.info_link,
            ai_summary: self.ai_summary,
            ..Book::default()
        }
    }
}

#[derive(FromRow)]
struct AuthorRecord {
    pub book_id: i64,
    pub author_id: i64,
    pub user_id: Option<i64>,
    pub name: String,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl AuthorRecord {
    pub fn into_author(self) -> Author {
        Author {
            author_id: self.author_id,
            user_id: self.user_id,
            name: self.name,
            bio: self.bio,
            website: self.website,
            created_at: self.created_at,
        }
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
            "info_link",
            "ai_summary"
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

        book.authors = if !authors.is_empty() { Some(authors) } else { None };
        book.reviews = if !reviews.is_empty() { Some(reviews) } else { None };
        Ok(Some(book))
    } else {
        Ok(None)
    }
}

pub async fn get_books(db_conn: &PgPool, ids: &[i64]) -> Result<Vec<Book>, Error> {
    let mut tx = db_conn.begin().await?;

    let book_records = sqlx::query_as!(
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
            "info_link",
            "ai_summary"
        FROM "books"
        WHERE "book_id" = ANY($1);
        "#,
        ids
    )
        .fetch_all(&mut *tx)
        .await?;

    if !book_records.is_empty() {
        let mut bookid2authors: HashMap<i64, Vec<Author>> = HashMap::new();
        let author_records = sqlx::query_as!(
            AuthorRecord,
            r#"
            SELECT "ba"."book_id", "a"."author_id", "user_id", "name", "bio", "website", "created_at"
            FROM "book_authors" AS "ba"
                JOIN "authors" AS "a" ON "ba"."author_id" = "a"."author_id"
            WHERE "ba"."book_id" = ANY($1)
            ORDER BY "ba"."book_id";
            "#,
            ids
        )
            .fetch_all(&mut *tx)
            .await?;

        author_records.into_iter()
            .for_each(|record| {
                bookid2authors.entry(record.book_id).or_default().push(record.into_author());
            });

        let mut bookid2reviews: HashMap<i64, Vec<Review>> = HashMap::new();
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
            WHERE "r"."book_id" = ANY($1);
            "#,
            ids
        )
            .fetch_all(&mut *tx)
            .await?;

        reviews.into_iter()
            .for_each(|review| {
                /* Since we filter the reviews by book IDs, each review must have a non-null book_id and thus unwrap() here won't panic. */
                bookid2reviews.entry(review.book_id.unwrap()).or_default().push(review);
            });
 
        let mut books: Vec<Book> = Vec::with_capacity(book_records.len());
        
        book_records.into_iter()
            .for_each(|record| {
                let mut book = record.into_book();

                book.authors = bookid2authors.remove(&book.book_id);
                book.reviews = bookid2reviews.remove(&book.book_id);
                books.push(book);
            });
        Ok(books)
    } else {
        Ok(Vec::new())
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
        SELECT "user_id", "email", "display_name", "role", "bio", "avatar", "website", "created_at", "preferred_topics"
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
            "helpful_yes" + (SELECT COUNT(*) FROM "review_votes" WHERE "review_id" = $1 AND "vote" = true)::integer AS "upvotes",
            "helpful_total" + (SELECT COUNT(*) FROM "review_votes" WHERE "review_id" = $1)::integer - "helpful_yes" - (SELECT COUNT(*) FROM "review_votes" WHERE "review_id" = $1 AND "vote" = true)::integer AS "downvotes",
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
