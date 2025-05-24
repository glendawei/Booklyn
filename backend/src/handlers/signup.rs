use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::AppData;
use crate::handlers::users::User;
use crate::error::Error;

#[derive(Deserialize, ToSchema)]
pub struct Signup {
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
}

#[utoipa::path(
    tag = "signup",
    request_body = Signup,
    responses(
        (status = 201, description = "Signup succeed.", body = User),
        (status = 409, description = "Conflict. Maybe the email is already signed up.", body = String),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[post("/signup")]
pub async fn signup(data: web::Data<AppData>, body: web::Json<Signup>) -> Result<impl Responder, Error> {
    let signup = body.into_inner();
    let mut tx = data.db_conn.begin().await?;

    match sqlx::query_as!(
        User,
        r#"
        INSERT INTO "users" ("email", "password_hash", "display_name", "role", "bio", "avatar", "website", "created_at")
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
        ON CONFLICT ("email") DO NOTHING
        RETURNING "user_id", "email", "display_name", "role", "bio", "avatar", "website", "created_at";
        "#,
        signup.email,
        signup.password_hash,
        signup.display_name,
        signup.role,
        signup.bio,
        signup.avatar,
        signup.website
    )
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(user) => {
            tx.commit().await?;
            Ok(HttpResponse::Created().json(user))
        },
        None => Ok(HttpResponse::Conflict().body("The email is already signed up."))
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(signup);   
}
