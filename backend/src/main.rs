mod handlers;
mod database;
mod api_doc;

use std::net::Ipv4Addr;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use api_doc::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run();

    log::info!("API documentation is available at http://localhost:8080/swagger-ui/");

    server.await
}
