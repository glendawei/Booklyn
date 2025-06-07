mod helpers;

use actix_web::test;
use actix_web::http::StatusCode;
use helpers::setup;
use booklyn_backend::handlers::{
    signup::Signup,
    login::Login,
    users::{
        User,
        UpdateUser,
        ReadingListItem,
        CreateReadingListItem,
        UpdateReadingListItem,
    },
};

#[actix_web::test]
async fn test_add_user() {
    let app = setup().await;
    let signup = Signup {
        display_name: "test_user_1".into(),
        email: "test1@gmail.com".into(),
        password_hash: "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f".into(), // sha256 from "12345678"
        role: "reader".into(),
        bio: Some("A test user.".into()),
        avatar: None,
        website: None,
        preferred_topics: vec!["Fiction".into()],
    };
    let req = test::TestRequest::post().uri("/signup").set_json(signup).to_request();
    let new_user: User = test::call_and_read_body_json(&app, req).await;

    assert_eq!(new_user.display_name, "test_user_1");
    assert_eq!(new_user.email.unwrap(), "test1@gmail.com");

    let login = Login {
        email: "test1@gmail.com".into(),
        password_hash: "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f".into(),
    };
    let req = test::TestRequest::post().uri("/login").set_json(login).to_request();
    let user: User = test::call_and_read_body_json(&app, req).await;

    assert_eq!(user.display_name, "test_user_1");
    assert_eq!(user.email.unwrap(), "test1@gmail.com");
    assert_eq!(user.preferred_topics.unwrap(), vec!["Fiction"]);    
}

#[actix_web::test]
async fn test_user_basic() {
    let app = setup().await;

    let display_name = "test_user_2";
    let email = "test2@gmail.com";
    /* Signup */
    let signup = Signup {
        display_name: display_name.to_string(),
        email: email.to_string(),
        password_hash: "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f".into(), // sha256 from "12345678"
        role: "reader".into(),
        bio: Some("A test user.".into()),
        avatar: None,
        website: None,
        preferred_topics: vec!["Fiction".into()],
    };
    let req = test::TestRequest::post().uri("/signup").set_json(signup).to_request();
    let user: User = test::call_and_read_body_json(&app, req).await;
    let user_id = user.user_id;
    
    /* Get user info by id */
    let uri = format!("/users/{}", user_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let user: User = test::call_and_read_body_json(&app, req).await;

    assert_eq!(user.user_id, user_id);
    assert_eq!(user.display_name, display_name);
    assert_eq!(user.email.unwrap(), email);

    /* Update user info */
    let display_name = "test_user_2_updated";
    let preferred_topics = vec![String::from("Fiction"), String::from("Science")];
    let update_user = UpdateUser {
        display_name: Some(display_name.to_string()),
        bio: None,
        avatar: None,
        website: None,
        preferred_topics: Some(preferred_topics.clone()),
    };
    let uri = format!("/users/{}", user_id);
    let req = test::TestRequest::patch().uri(uri.as_str()).set_json(update_user).to_request();
    let user: User = test::call_and_read_body_json(&app, req).await;

    assert_eq!(user.display_name, display_name);
    assert_eq!(user.preferred_topics.unwrap(), preferred_topics);
}

#[actix_web::test]
async fn test_reading_list_basic() {
    let app = setup().await;
    
    /* Signup */
    let signup = Signup {
        display_name: "test_user_3".into(),
        email: "test3@gmail.com".into(),
        password_hash: "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f".into(), // sha256 from "12345678"
        role: "reader".into(),
        bio: Some("A test user.".into()),
        avatar: None,
        website: None,
        preferred_topics: vec!["Fiction".into()],
    };
    let req = test::TestRequest::post().uri("/signup").set_json(signup).to_request();
    let user: User = test::call_and_read_body_json(&app, req).await;
    let user_id = user.user_id;

    let book_id = 1i64;
    let status = "new";
    let note = "must read";
    /* Create reading list item */
    let create_item = CreateReadingListItem {
        book_id,
        status: status.into(),
        note: Some(note.into()),
    };
    let uri = format!("/users/{}/reading-list", user_id);
    let req = test::TestRequest::post().uri(uri.as_str()).set_json(create_item).to_request();
    let item: ReadingListItem = test::call_and_read_body_json(&app, req).await;
    let item_id = item.item_id;

    assert_eq!(item.user_id.unwrap(), user_id);
    assert_eq!(item.book_id.unwrap(), book_id);
    assert_eq!(item.status, status);
    assert_eq!(item.note.unwrap(), note);

    let uri = format!("/users/{}/reading-list", user_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let items: Vec<ReadingListItem> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(items.len(), 1);

    let status = "start reading";
    let note = "stoped at p.100";
    /* Update reading list item */
    let update_item = UpdateReadingListItem {
        status: Some(status.into()),
        note: Some(note.into()),
    };
    let uri = format!("/users/{}/reading-list/{}", user_id, item_id);
    let req = test::TestRequest::patch().uri(uri.as_str()).set_json(update_item).to_request();
    let item: ReadingListItem = test::call_and_read_body_json(&app, req).await;

    assert_eq!(item.status, status);
    assert_eq!(item.note.unwrap(), note);

    /* Delete reading list item */
    let uri = format!("/users/{}/reading-list/{}", user_id, item_id);
    let req = test::TestRequest::delete().uri(uri.as_str()).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let uri = format!("/users/{}/reading-list", user_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let items: Vec<ReadingListItem> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(items.len(), 0);
}
