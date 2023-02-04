use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}
