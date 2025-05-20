use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use time::OffsetDateTime;
use utoipa::{schema, ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct Signup {
    pub user_id: i64,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub website: Option<String>,
    /* Time format: "yyyy-mm-dd HH:MM:SS[+|-][hh]" */
    pub created_at: String,
}

#[utoipa::path(
    tag = "signup",
    request_body = Signup,
    responses(
        (status = 201, description = "Signup succeed.", body = User),
        (status = 400, description = "Invalid reqeust. Maybe the timestamp format is invalid or the email is already signuped.")
    )
)]
#[post("/signup")]
pub async fn signup(body: web::Json<Signup>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(signup);   
}
