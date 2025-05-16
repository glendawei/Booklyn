use actix_web::{middleware, App, HttpServer};
use Booklyn_backend::routes::*;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(login)
            .service(logout)
            .service(get_books)
            .service(get_book_by_id)
            .service(search_book_by_query)
            .service(create_review)
            .service(update_review)
            .service(delete_review_by_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
