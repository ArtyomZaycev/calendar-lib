// NOT IMPLEMENTED

pub mod types {
    use std::num::{NonZeroU16, NonZeroU32};

    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WeekdayMask(pub u8);

    impl Default for WeekdayMask {
        fn default() -> Self {
            Self(0b1111111)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Schedule {
        pub id: i32,
        pub user_id: i32,
        pub start: NaiveDateTime,
        pub weekday_filter: WeekdayMask,
        pub day_period: Option<NonZeroU32>,
        pub time_period: Option<NonZeroU16>,
        pub event_duration: u16,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewSchedule {
        pub user_id: i32,
        pub start: NaiveDateTime,
        pub weekday_filter: WeekdayMask,
        pub day_period: Option<NonZeroU32>,
        pub time_period: Option<NonZeroU16>,
        pub event_duration: u16,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateSchedule {
        pub id: i32,
        pub user_id: Option<i32>,
        pub start: Option<NaiveDateTime>,
        pub weekday_filter: Option<WeekdayMask>,
        pub day_period: Option<Option<NonZeroU32>>,
        pub time_period: Option<Option<NonZeroU16>>,
        pub event_duration: Option<u16>,
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

pub mod update {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub static METHOD: Method = Method::PATCH;
    pub static PATH: &str = "schedule";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        pub upd_schedule: UpdateSchedule,
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
