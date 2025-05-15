use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Clone, Serialize)]
pub struct Review {
    pub user: String,
    pub id: u64,
    pub rating: u8,
    pub reviewTime: String,
    pub reviewContent: String,
}
