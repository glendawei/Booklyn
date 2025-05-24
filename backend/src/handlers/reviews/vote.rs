use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::AppData;
use crate::error::Error;

#[derive(Serialize, ToSchema)]
pub struct Vote {
    pub user_id: i64,
    pub review_id: i64,
    pub vote: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateVote {
    pub user_id: i64,
    pub helpful: bool,
}

#[utoipa::path(
    tag = "reviews",
    params(
        ("review_id" = i64, Path, description = "ID for the review")
    ),
    responses(
        (status = 200, description = "Successful operation.", body = Vote),
        (status = 404, description = "User not found or review not found."),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/reviews/{review_id}/vote")]
pub async fn create_vote(
    data: web::Data<AppData>,
    id: web::Path<i64>,
    body: web::Json<CreateVote>
) -> Result<impl Responder, Error> {
    let review_id = id.into_inner();
    let create_vote = body.into_inner();
    let mut tx = data.db_conn.begin().await?;
    
    match sqlx::query_as!(
        Vote,
        r#"
        INSERT INTO "review_votes" ("user_id", "review_id", "vote")
        SELECT $1, $2, $3
        WHERE
            EXISTS (
                SELECT "user_id"
                FROM "users"
                WHERE "user_id" = $1
            ) AND
            EXISTS (
                SELECT "review_id"
                FROM "reviews"
                WHERE "review_id" = $2
            )
        ON CONFLICT ("user_id", "review_id") DO UPDATE SET
            "vote" = $3
        RETURNING "user_id", "review_id", "vote";
        "#,
        create_vote.user_id,
        review_id,
        create_vote.helpful
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(vote) => {
            sqlx::query!(
                r#"
                UPDATE "reviews" SET
                    "helpful_total" = "helpful_total" + 1,
                    "helpful_yes" = "helpful_yes" + $1
                WHERE "review_id" = $2;
                "#,
                vote.vote as i64,
                review_id
            )
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;
            Ok(HttpResponse::Ok().json(vote))
        },
        None => Ok(HttpResponse::NotFound().finish())
    }
}
