pub mod types {
    pub(super) use crate::api::utils::*;
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};
}

pub mod load_array {
    use super::types::*;

    use crate::api::roles::types::Role;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "user_roles";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub user_id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<Role>,
    }
}

pub mod insert {
    use super::types::*;

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "user_role";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub user_id: i32,
        pub role_id: i32,
    }

    pub type Response = EmptyResponse;
}

pub mod delete {
    use super::types::*;

    pub const METHOD: Method = Method::DELETE;
    pub const PATH: &str = "user_role";

    pub type Args = DeleteByIdQuery;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    pub type Response = EmptyResponse;
    pub type BadRequestResponse = DeleteBadRequestResponse;
}
