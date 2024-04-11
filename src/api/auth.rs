pub mod types {
    pub(super) use crate::api::utils::EmptyResponse;
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};

    // TODO: User id field
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessLevel {
        pub level: i32,
        pub name: String,
        pub edit_rights: bool,
    }

    impl AccessLevel {
        pub const MAX_LEVEL: i32 = 1000;

        pub fn is_max_level(&self) -> bool {
            self.level >= Self::MAX_LEVEL
        }
        pub fn is_full(&self) -> bool {
            self.is_max_level() && self.edit_rights
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewPassword {
        pub name: String,
        pub password: String,
    }
}

pub mod logout {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "auth/logout";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    pub type Response = EmptyResponse;
}

pub mod login {
    use super::types::*;
    use crate::api::utils::User;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "auth/login";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        UserNotFound, // Both for unknown name & wrong password
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub user: User,
        pub access_level: AccessLevel,
        pub jwt: String,
    }
}

pub mod login_by_key {
    use super::types::*;
    use crate::api::utils::User;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "auth/login_key";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub user: User,
        pub access_level: AccessLevel,
        pub jwt: String,
    }
}

pub mod register {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "auth/register";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub name: String,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        EmailAlreadyUsed,
    }

    pub type Response = EmptyResponse;
}

pub mod new_password {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "auth/new_password";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub user_id: i32,
        pub access_level: i32,
        pub viewer_password: Option<NewPassword>,
        pub editor_password: Option<NewPassword>,
    }

    pub type Response = EmptyResponse;
}

pub mod load_access_levels {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "auth/load_access_levels";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<AccessLevel>,
    }
}
