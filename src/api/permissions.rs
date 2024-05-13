pub mod types {
    pub(super) use crate::api::utils::*;
    pub(super) use http::Method;
    pub(super) use serde::{Deserialize, Serialize};

    use crate::api::auth::types::AccessLevel;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct TablePermissions {
        pub view: bool,
        pub edit: bool,
        pub create: bool,
        pub delete: bool,
    }

    impl TablePermissions {
        pub const FULL: Self = Self {
            view: true,
            edit: true,
            create: true,
            delete: true,
        };

        pub const NONE: Self = Self {
            view: false,
            edit: false,
            create: false,
            delete: false,
        };

        pub fn is_none(&self) -> bool {
            *self == Self::NONE
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct Permissions {
        pub access_level: i32,
        pub access_levels: TablePermissions,
        pub events: TablePermissions,
        pub event_templates: TablePermissions,
        pub schedules: TablePermissions,
        pub allow_share: bool,
    }

    impl Permissions {
        pub const FULL: Self = Self {
            access_level: AccessLevel::MAX_LEVEL,
            access_levels: TablePermissions::FULL,
            events: TablePermissions::FULL,
            event_templates: TablePermissions::FULL,
            schedules: TablePermissions::FULL,
            allow_share: true,
        };

        pub const NONE: Self = Self {
            access_level: AccessLevel::MIN_LEVEL - 1,
            access_levels: TablePermissions::NONE,
            events: TablePermissions::NONE,
            event_templates: TablePermissions::NONE,
            schedules: TablePermissions::NONE,
            allow_share: false,
        };

        pub fn is_none(&self) -> bool {
            *self == Self::NONE
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct GrantedPermission {
        pub id: i32,
        pub giver_user_id: i32,
        pub receiver_user_id: i32,
        pub permissions: Permissions,
    }
}

pub mod load_array {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "load_granted_permissions";

    pub type Args = LoadArrayQuery;
    pub type Response = Vec<GrantedPermission>;
}
