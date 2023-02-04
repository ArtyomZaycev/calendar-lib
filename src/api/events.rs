use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub access_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub access_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub access_level: i32,
}

pub mod load {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Get;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub event: super::Event,
    }
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
        pub array: Vec<super::Event>,
    }
}

pub mod insert {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Post;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub new_event: super::NewEvent,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod update {
    use crate::api::utils::Method;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Patch;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub user_id: Option<i32>,
        pub name: Option<String>,
        pub description: Option<Option<String>>,
        pub start: Option<NaiveDateTime>,
        pub end: Option<NaiveDateTime>,
        pub access_level: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod delete {
    use crate::api::utils::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::Delete;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}
