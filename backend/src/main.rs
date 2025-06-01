mod handlers;
mod database;
mod api_doc;
mod error;

use std::net::Ipv4Addr;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use env_logger::Env;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use api_doc::ApiDoc;
use actix_cors::Cors;

const DB_CONN_MAX: u32 = 32;

struct AppData {
    db_conn: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let db_url =
        std::env::var("DATABASE_URL").expect("Please setup database URL with environment variable DATABASE_URL.");
    let pool = PgPoolOptions::new()
        .max_connections(DB_CONN_MAX)
        .connect(&db_url)
        .await
        .expect("Failed to connect database.");

    log::info!("connect to database at {}", db_url);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData { db_conn: pool.clone() }))
            .wrap(
                /* Just for overcoming tests, the settings are not safe enough for production */
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
        })
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run();
    
    log::info!("start HTTP server at http://localhost:8080");
    log::info!("API documentation is available at http://localhost:8080/swagger-ui/");

    server.await
}
