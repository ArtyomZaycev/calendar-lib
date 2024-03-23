
pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserMemoryUsageData {
        pub events_count: usize,
        pub passwords_count: usize,
        pub schedules_count: usize,
        pub event_plans_count: usize,
        pub event_templates_count: usize,
        pub bytes: usize,
    }
}

// ADMIN ONLY REQUEST
pub mod load_user_memory_usage {
    use super::types::*;
    use http::Method;
    use serde::{Deserialize, Serialize};

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "load_user_memory_usage";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {
        pub user_id: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub data: UserMemoryUsageData,
    }
}
