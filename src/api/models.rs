use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    pub data: Vec<User>,
    pub count: usize,
}