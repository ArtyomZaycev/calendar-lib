pub mod types {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    use crate::api::utils::UpdateOption;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[repr(i8)]
    pub enum EventVisibility {
        HideAll,         // No access - hide completelly
        HideName,        // No access - show time
        HideDescription, // No access - show name & time
        Show,            // No access - show anyway
    }

    impl TryFrom<i8> for EventVisibility {
        type Error = ();

        fn try_from(value: i8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(EventVisibility::HideAll),
                1 => Ok(EventVisibility::HideName),
                2 => Ok(EventVisibility::HideDescription),
                3 => Ok(EventVisibility::Show),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Event {
        pub id: i32,
        pub user_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub start: NaiveDateTime,
        pub end: NaiveDateTime,
        pub access_level: i32,
        pub visibility: EventVisibility,
        pub plan_id: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewEvent {
        pub user_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub start: NaiveDateTime,
        pub end: NaiveDateTime,
        pub access_level: i32,
        pub visibility: EventVisibility,
        pub plan_id: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateEvent {
        pub id: i32,
        pub name: UpdateOption<String>,
        pub description: UpdateOption<Option<String>>,
        pub start: UpdateOption<NaiveDateTime>,
        pub end: UpdateOption<NaiveDateTime>,
        pub access_level: UpdateOption<i32>,
        pub visibility: UpdateOption<EventVisibility>,
        pub plan_id: UpdateOption<Option<i32>>,
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
    pub struct Response {
        pub value: Event,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        NotFound,
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
    pub struct Response {}
}
