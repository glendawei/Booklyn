use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use utoipa::ToSchema;

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
        (status = 401, description = "Authentication failed."),
        (status = 404, description = "Account not found.")
    )
)]
#[post("/login")]
pub async fn login(body: web::Json<Login>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
