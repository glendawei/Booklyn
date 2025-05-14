use actix_web::{App, HttpServer};
use Booklyn::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
