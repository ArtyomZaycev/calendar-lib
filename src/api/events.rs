pub mod types {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Event {
        pub id: i32,
        pub user_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub start: NaiveDateTime,
        pub end: NaiveDateTime,
        pub access_level: i32,
        pub plan_id: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewEvent {
        pub name: String,
        pub description: Option<String>,
        pub start: NaiveDateTime,
        pub end: NaiveDateTime,
        pub access_level: i32,
        pub plan_id: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateEvent {
        pub id: i32,
        pub user_id: Option<i32>,
        pub name: Option<String>,
        pub description: Option<Option<String>>,
        pub start: Option<NaiveDateTime>,
        pub end: Option<NaiveDateTime>,
        pub access_level: Option<i32>,
        pub plan_id: Option<Option<i32>>,
    }
}

pub mod load {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::GET;
    pub static PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub event: Event,
    }
}

pub mod load_array {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::GET;
    pub static PATH: &str = "events";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<Event>,
    }
}

pub mod insert {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub new_event: NewEvent,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod update {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::PATCH;
    pub static PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub upd_event: UpdateEvent,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod delete {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::DELETE;
    pub static PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}
