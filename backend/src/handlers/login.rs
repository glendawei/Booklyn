use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::AppData;
use crate::handlers::users::User;
use crate::error::Error;

#[derive(Deserialize, ToSchema)]
pub struct Login {
    pub email: String,
    pub password_hash: String,
}

#[utoipa::path(
    tag = "login",
    request_body = Login,
    responses(
        (status = 200, description = "Login succeed.", body = User),
        (status = 401, description = "Authentication failed.", body = String),
        (status = 500, description = "Internal server error.", body = String)
    )
)]
#[post("/login")]
pub async fn login(data: web::Data<AppData>, body: web::Json<Login>) -> Result<impl Responder, Error> {
    match sqlx::query_as!(
        User,
        r#"
        SELECT "user_id", "email", "display_name", "role", "bio", "avatar", "website", "created_at", "preffered_topics"
        FROM "users" 
        WHERE "email" = $1 AND "password_hash" = $2;
        "#,
        body.email,
        body.password_hash
    )
        .fetch_optional(&data.db_conn)
        .await?
    {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::Unauthorized().body("The account does not exist or the password is wrong."))
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
