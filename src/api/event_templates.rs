pub mod types {
    pub(super) use crate::api::utils::{DeleteByIdQuery, EmptyResponse, LoadByIdQuery};
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};

    use crate::api::utils::UpdateOption;
    use std::time::Duration;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EventTemplate {
        pub id: i32,
        pub user_id: i32,
        pub name: String,
        pub event_name: String,
        pub event_description: Option<String>,
        pub duration: Duration,
        pub access_level: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewEventTemplate {
        pub user_id: i32,
        pub name: String,
        pub event_name: String,
        pub event_description: Option<String>,
        pub duration: Duration,
        pub access_level: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateEventTemplate {
        pub id: i32,
        pub name: UpdateOption<String>,
        pub event_name: UpdateOption<String>,
        pub event_description: UpdateOption<Option<String>>,
        pub duration: UpdateOption<Duration>,
        pub access_level: UpdateOption<i32>,
    }
}

pub mod load {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "event_template";

    pub type Args = LoadByIdQuery;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub value: EventTemplate,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        NotFound,
    }
}
pub mod load_array {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "event_templates";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Response = Vec<EventTemplate>;
}

pub mod insert {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "event_template";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = NewEventTemplate;

    pub type Response = EmptyResponse;
}

pub mod update {
    use super::types::*;

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "event_template";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = UpdateEventTemplate;

    pub type Response = EmptyResponse;
}

pub mod delete {
    use super::types::*;

    pub const METHOD: Method = Method::DELETE;
    pub const PATH: &str = "event_template";

    pub type Args = DeleteByIdQuery;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    pub type Response = EmptyResponse;
}
