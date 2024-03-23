pub mod types {
    use serde::{Deserialize, Serialize};

    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Role {
        SuperAdmin = 1,
        Admin = 2,
    }
}

pub mod load_array {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "roles";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<Role>,
    }
}
