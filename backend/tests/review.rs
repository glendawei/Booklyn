mod helpers;

use actix_web::http::StatusCode;
use actix_web::test;
use booklyn_backend::handlers::{
    signup::Signup,
    users::User,
    reviews::{
        Comment,
        CreateComment,
        CreateVote,
        Review,
        UpdateComment,
    },
};
use helpers::setup;

#[actix_web::test]
async fn test_get_review() {
    let app = setup().await;
    let review_id = 1;
    let uri = format!("/reviews/{}", review_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let review: Review = test::call_and_read_body_json(&app, req).await;

    assert_eq!(review.review_id, review_id);
}

#[actix_web::test]
async fn test_comment_basic() {
    let app = setup().await;

    let review_id = 1i64;
    let content = "This book is too expensive.";
    /* Create a new comment */
    let create_comment = CreateComment {
        user_id: None,
        parent_id: None,
        content: content.into(),
    };
    let uri = format!("/reviews/{}/comments", review_id);
    let req = test::TestRequest::post()
        .uri(uri.as_str())
        .set_json(create_comment)
        .to_request();
    let comment: Comment = test::call_and_read_body_json(&app, req).await;
    let comment_id = comment.comment_id;

    assert_eq!(comment.review_id.unwrap(), review_id);
    assert_eq!(comment.content, content);

    /* Get an existing comment */
    let uri = format!("/reviews/{}/comments/{}", review_id, comment_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let comment: Comment = test::call_and_read_body_json(&app, req).await;

    assert_eq!(comment.comment_id, comment_id);

    /* Update an existing comment */
    let content = "This book is not that expensive now.";
    let update_comment = UpdateComment {
        content: content.into(),
    };
    let uri = format!("/reviews/{}/comments/{}", review_id, comment_id);
    let req = test::TestRequest::patch()
        .uri(uri.as_str())
        .set_json(update_comment)
        .to_request();
    let comment: Comment = test::call_and_read_body_json(&app, req).await;

    assert_eq!(comment.content, content);

    /* Delete an existing comment */
    let uri = format!("/reviews/{}/comments/{}", review_id, comment_id);
    let req = test::TestRequest::delete().uri(uri.as_str()).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let uri = format!("/reviews/{}/comments", review_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let comments: Vec<Comment> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(comments.len(), 0);
}

#[actix_web::test]
async fn test_vote_basic() {
    let app = setup().await;

    /* Signup a new user */
    let signup = Signup {
        display_name: "test_user_5".into(),
        email: "test5@gmail.com".into(),
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

    /* Get review */
    let review_id = 1i64;
    let uri = format!("/reviews/{}", review_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let review: Review = test::call_and_read_body_json(&app, req).await;
    let orig_upvotes = review.upvotes.unwrap_or(0);
    let orig_downvotes = review.downvotes.unwrap_or(0);

    /* Upvote */
    let create_vote = CreateVote {
        user_id: user_id,
        helpful: true,
    };
    let uri = format!("/reviews/{}/vote", review_id);
    let req = test::TestRequest::post().uri(uri.as_str()).set_json(create_vote).to_request();
    let _ = test::call_service(&app, req).await;

    let uri = format!("/reviews/{}", review_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let review: Review = test::call_and_read_body_json(&app, req).await;

    assert_eq!(review.upvotes.unwrap_or(0), orig_upvotes + 1);
    assert_eq!(review.downvotes.unwrap_or(0), orig_downvotes);

    /* Downvote */
    let create_vote = CreateVote {
        user_id: user_id,
        helpful: false,
    };
    let uri = format!("/reviews/{}/vote", review_id);
    let req = test::TestRequest::post().uri(uri.as_str()).set_json(create_vote).to_request();
    let _ = test::call_service(&app, req).await;

    let uri = format!("/reviews/{}", review_id);
    let req = test::TestRequest::get().uri(uri.as_str()).to_request();
    let review: Review = test::call_and_read_body_json(&app, req).await;

    assert_eq!(review.upvotes.unwrap_or(0), orig_upvotes);
    assert_eq!(review.downvotes.unwrap_or(0), orig_downvotes + 1);
}
