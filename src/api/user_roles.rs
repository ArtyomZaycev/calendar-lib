pub mod load_array {
    use http::Method;
    use serde::{Deserialize, Serialize};

    use crate::api::roles::types::Role;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "user_roles";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub user_id: Option<i32>, // Admin can load roles of any user
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<Role>,
    }
}

pub mod insert {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "user_role";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub user_id: i32,
        pub role_id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod delete {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub const METHOD: Method = Method::DELETE;
    pub const PATH: &str = "user_role";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}
