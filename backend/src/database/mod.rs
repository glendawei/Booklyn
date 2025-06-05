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

pub async fn get_books(db_conn: &PgPool, ids: &[i64]) -> Result<Vec<Book>, Error> {
    let mut tx = db_conn.begin().await?;
    let id_list = ids.into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let query_books = format!(
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
        WHERE "book_id" IN ({});
        "#,
        id_list
    );
    let book_records: Vec<BookRecord> = sqlx::query_as(&query_books)
        .fetch_all(&mut *tx)
        .await?;

    if book_records.len() > 0 {
        let mut bookid2authors: HashMap<i64, Vec<Author>> = HashMap::new();
        let query_authors = format!(
            r#"
            SELECT "ba"."book_id", "a"."author_id", "user_id", "name", "bio", "website", "created_at"
            FROM "book_authors" AS "ba"
                JOIN "authors" AS "a" ON "ba"."author_id" = "a"."author_id"
            WHERE "ba"."book_id" IN ({})
            ORDER BY "ba"."book_id";
            "#,
            id_list
        );
        let author_records: Vec<AuthorRecord> = sqlx::query_as(&query_authors)
            .fetch_all(&mut *tx)
            .await?;

        author_records.into_iter()
            .for_each(|record| {
                if bookid2authors.contains_key(&record.book_id) {
                    bookid2authors.get_mut(&record.book_id).unwrap().push(record.into_author());
                } else {
                    bookid2authors.insert(record.book_id, vec![record.into_author()]);
                }
            });

        let mut bookid2reviews: HashMap<i64, Vec<Review>> = HashMap::new();
        let query_reviews = format!(
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
            WHERE "r"."book_id" IN ({});
            "#,
            id_list
        );
        let reviews: Vec<Review> = sqlx::query_as(&query_reviews)
            .fetch_all(&mut *tx)
            .await?;

        reviews.into_iter()
            .for_each(|r| {
                let book_id = r.book_id.unwrap();
                
                if bookid2reviews.contains_key(&book_id) {
                    bookid2reviews.get_mut(&book_id).unwrap().push(r);
                } else {
                    bookid2reviews.insert(book_id, vec![r]);
                }
            });
 
        let mut books: Vec<Book> = Vec::with_capacity(book_records.len());
        
        for book_record in book_records.into_iter() {
            let mut book = book_record.into_book();

            book.authors = match bookid2authors.get_mut(&book.book_id) {
                Some(authors) => Some(std::mem::take(authors)),
                None => None,
            };
            book.reviews = match bookid2reviews.get_mut(&book.book_id) {
                Some(reviews) => Some(std::mem::take(reviews)),
                None => None
            };
            books.push(book);
        }
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
