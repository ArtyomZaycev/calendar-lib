pub mod types {
    pub(super) use crate::api::utils::*;
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};

    use chrono::{NaiveDate, NaiveTime, Weekday};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EventPlan {
        pub id: i32,
        pub weekday: Weekday,
        pub time: NaiveTime,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Schedule {
        pub id: i32,
        pub user_id: i32,
        pub template_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub first_day: NaiveDate,
        pub last_day: Option<NaiveDate>,
        pub access_level: i32,
        pub event_plans: Vec<EventPlan>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewEventPlan {
        pub weekday: Weekday,
        pub time: NaiveTime,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewSchedule {
        pub user_id: i32,
        pub template_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub first_day: NaiveDate,
        pub last_day: Option<NaiveDate>,
        pub access_level: i32,
        pub events: Vec<NewEventPlan>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateSchedule {
        pub id: i32,
        //pub template_id: UpdateOption<i32>,
        pub name: UpdateOption<String>,
        pub description: UpdateOption<Option<String>>,
        pub first_day: UpdateOption<NaiveDate>,
        pub last_day: UpdateOption<Option<NaiveDate>>,
        pub access_level: UpdateOption<i32>,
        pub delete_events: Vec<i32>,
        pub new_events: Vec<NewEventPlan>,
    }
}

pub mod load {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "schedule";

    pub type Args = LoadByIdQuery;
    pub type Response = Schedule;
    pub type BadRequestResponse = LoadByIdBadRequestResponse;
}

pub mod load_array {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "schedules";

    pub type Args = LoadArrayQuery;
    pub type Response = Vec<Schedule>;
}

pub mod insert {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "schedule";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = NewSchedule;

    pub type Response = EmptyResponse;
}

pub mod update {
    use super::types::*;

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "schedule";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = UpdateSchedule;

    pub type Response = EmptyResponse;
    pub type BadRequestResponse = UpdateBadRequestResponse;
}

pub mod delete {
    use super::types::*;

    pub const METHOD: Method = Method::DELETE;
    pub const PATH: &str = "schedule";

    pub type Args = DeleteByIdQuery;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    pub type Response = EmptyResponse;
    pub type BadRequestResponse = DeleteBadRequestResponse;
}
