pub mod types {
    use serde::{Deserialize, Serialize};

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
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "auth/logout";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod login {
    use super::types::*;
    use crate::api::utils::User;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "auth/login";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub user: User,
        pub access_level: AccessLevel,
        pub key: Vec<u8>,
    }
}

pub mod register {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "auth/register";

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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod new_password {
    use http::Method;
    use serde::{Deserialize, Serialize};

    use super::types::NewPassword;

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "auth/new_password";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub user_id: i32,
        pub access_level: i32,
        pub viewer_password: Option<NewPassword>,
        pub editor_password: Option<NewPassword>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod load_access_levels {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::GET;
    pub static PATH: &str = "auth/load_access_levels";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<AccessLevel>,
    }
}
