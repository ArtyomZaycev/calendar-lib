// ADMIN ONLY REQUESTS

pub mod types {
    pub(super) use crate::api::utils::{
        LoadByIdBadRequestResponse, LoadByIdQuery, User
    };
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};
}

pub mod load_ids {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "user_ids";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<i32>,
    }
}

pub mod load {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "user";

    pub type Args = LoadByIdQuery;
    pub type Response = User;
    pub type BadRequestResponse = LoadByIdBadRequestResponse;
}

pub mod load_array {
    use crate::api::utils::User;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "users";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Response = Vec<User>;
}
