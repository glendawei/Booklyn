use utoipa::OpenApi;
use crate::handlers::{
    books::{self, SearchQuery, Book},
    reviews::{self, Review, Comment, CreateComment, UpdateComment, Vote, CreateVote},
    authors::{self, Author},
    users::{self, User, UpdateUser, ReadingListItem, CreateReadingListItem, UpdateReadingListItem},
    signup::{self, Signup},
    login::{self, Login}
};

#[derive(OpenApi)]
#[openapi(
    paths(
        books::get_books,
        books::get_book_by_id,
        authors::get_authors_by_id,
        users::get_users_by_id,
        users::update_user_by_id,
        users::get_reading_list_items,
        users::create_reading_list_item,
        users::update_reading_list_item_by_id,
        users::delete_reading_list_item_by_id,
        signup::signup,
        login::login,
        reviews::get_review_by_id,
        reviews::get_comment_by_id,
        reviews::create_comment,
        reviews::update_comment_by_id,
        reviews::delete_comment_by_id,
        reviews::create_vote
    ),
    components(
        schemas(
            SearchQuery, 
            Book, 
            Review, 
            Author, 
            User, 
            UpdateUser, 
            ReadingListItem,
            CreateReadingListItem,
            UpdateReadingListItem,
            Signup,
            Login,
            Comment,
            CreateComment,
            UpdateComment,
            Vote,
            CreateVote
        )
    )
)]
pub struct ApiDoc;
