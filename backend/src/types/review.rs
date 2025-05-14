use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Clone, Serialize)]
pub struct Review {
    user: String,
    id: u64,
    rating: u8,
    reviewTime: String,
    reviewContent: String,
}
