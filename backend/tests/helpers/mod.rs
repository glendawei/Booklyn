use actix_http::Request;
use actix_service::Service;
use actix_web::test;
use actix_web::{App, Error, dev::ServiceResponse, web};
use booklyn_backend::{AppData, handlers};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn setup() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL_TEST")
        .expect("Please setup database URL with environment variable DATABASE_URL.");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("Failed to connect database.");

    test::init_service(
        App::new()
            .app_data(web::Data::new(AppData { db_conn: pool }))
            .configure(handlers::config),
    )
    .await
}
