use serde::{Deserialize, Serialize};

// TODO: Remove
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}
