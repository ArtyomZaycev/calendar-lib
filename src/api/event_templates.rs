pub mod types {
    use std::time::Duration;

    use serde::{Deserialize, Serialize};

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
}

pub mod load_array {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::GET;
    pub static PATH: &str = "event_templates";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<EventTemplate>,
    }
}

pub mod insert {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "event_template";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub new_event_template: NewEventTemplate,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod delete {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::DELETE;
    pub static PATH: &str = "event_template";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}