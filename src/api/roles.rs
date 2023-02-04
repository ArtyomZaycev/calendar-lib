use serde::{Deserialize, Serialize};

#[repr(i32)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin = 1,
    Admin = 2,
}

pub mod load_array {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Get;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<super::Role>,
    }
}
