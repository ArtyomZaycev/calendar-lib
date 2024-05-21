pub mod load {
    use http::Method;
    use serde::{Deserialize, Serialize};

    use crate::api::{
        auth::types::AccessLevel, event_templates::types::EventTemplate, events::types::Event,
        permissions::types::GrantedPermission, schedules::types::Schedule, utils::User,
    };

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "state";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub user_id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub users: Vec<User>,
        pub access_levels: Vec<AccessLevel>,
        pub events: Vec<Event>,
        pub event_templates: Vec<EventTemplate>,
        pub schedules: Vec<Schedule>,
        pub granted_permissions: Vec<GrantedPermission>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        UserNotFound,
    }
}
