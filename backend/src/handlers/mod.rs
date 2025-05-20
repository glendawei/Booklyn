pub mod signup;
pub mod login;
pub mod users;
pub mod books;
pub mod authors;
pub mod reviews;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.configure(signup::config)
        .configure(login::config)
        .configure(users::config)
        .configure(books::config)
        .configure(authors::config)
        .configure(reviews::config);
}
