mod helpers;

use actix_web::test;
use helpers::setup;
use booklyn_backend::handlers::books::Book;

#[actix_web::test]
async fn test_get_book_by_id() {
    let app = setup().await;
    let req = test::TestRequest::get().uri("/books/1").to_request();
    let book: Book = test::call_and_read_body_json(&app, req).await;
    
    assert_eq!(book.book_id, 1);
}

#[actix_web::test]
async fn test_get_books() {
    let app = setup().await;
    let req = test::TestRequest::get().uri("/books").to_request();
    let _: Vec<Book> = test::call_and_read_body_json(&app, req).await;
}
