pub mod types {
    use chrono::{NaiveDate, NaiveTime, Weekday};
    use serde::{Deserialize, Serialize};

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
        pub events: Vec<EventPlan>,
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
}

pub mod load_array {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::GET;
    pub static PATH: &str = "schedules";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub array: Vec<Schedule>,
    }
}

pub mod insert {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::POST;
    pub static PATH: &str = "schedule";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub new_schedule: NewSchedule,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}

pub mod delete {
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::DELETE;
    pub static PATH: &str = "schedule";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {}
}
