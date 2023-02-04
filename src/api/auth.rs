pub mod login {
    use crate::api::utils::*;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Post;

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
        pub access_level: i32,
        pub edit_rights: bool,
        pub key: Vec<u8>,
    }
}

pub mod register {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Post;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub name: String,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod new_password {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Post;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub new_password: String,
        pub access_level: i32,
        pub edit_right: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}
