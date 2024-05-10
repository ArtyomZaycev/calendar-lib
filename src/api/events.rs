pub mod types {
    pub(super) use crate::api::utils::*;
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};

    use chrono::NaiveDateTime;

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

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "event";

    pub type Args = LoadByIdQuery;
    pub type Response = Event;
    pub type BadRequestResponse = LoadByIdBadRequestResponse;
}

pub mod load_array {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "events";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Response = Vec<Event>;
}

pub mod insert {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = NewEvent;

    pub type Response = EmptyResponse;
}

pub mod update {
    use super::types::*;

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "event";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = UpdateEvent;

    pub type Response = EmptyResponse;
    pub type BadRequestResponse = UpdateBadRequestResponse;
}

pub mod delete {
    use super::types::*;

    pub const METHOD: Method = Method::DELETE;
    pub const PATH: &str = "event";
    pub type Args = DeleteByIdQuery;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    pub type Response = EmptyResponse;
    pub type BadRequestResponse = DeleteBadRequestResponse;
}
