pub mod types {
    use std::ops::BitOr;

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

    impl BitOr for TablePermissions {
        type Output = Self;
    
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                view: self.view || rhs.view,
                edit: self.edit || rhs.edit,
                create: self.create || rhs.create,
                delete: self.delete || rhs.delete,
            }
        }
    }

    impl BitOr for Permissions {
        type Output = Self;
    
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                access_level: self.access_level.max(rhs.access_level),
                access_levels: self.access_levels | rhs.access_levels,
                events: self.events | rhs.events,
                event_templates: self.event_templates | rhs.event_templates,
                schedules: self.schedules | rhs.schedules,
                allow_share: self.allow_share || rhs.allow_share,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct GrantedPermission {
        pub id: i32,
        pub giver_user_id: i32,
        pub receiver_user_id: i32,
        pub permissions: Permissions,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NewGrantedPermission {
        pub giver_user_id: i32,
        pub receiver_email: String,
        pub permissions: Permissions,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateGrantedPermission {
        pub id: i32,
        pub receiver_email: UpdateOption<String>,
        pub permissions: UpdateOption<Permissions>,
    }
}

pub mod load_array {
    use super::types::*;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "permissions";

    pub type Args = LoadArrayQuery;
    pub type Response = Vec<GrantedPermission>;
}

pub mod insert {
    use super::types::*;

    pub const METHOD: Method = Method::POST;
    pub const PATH: &str = "permission";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = NewGrantedPermission;
    pub type Response = EmptyResponse;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        UserEmailNotFound,
    }
}

pub mod update {
    use super::types::*;

    pub const METHOD: Method = Method::PATCH;
    pub const PATH: &str = "permission";

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Args {}

    pub type Body = UpdateGrantedPermission;
    pub type Response = EmptyResponse;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BadRequestResponse {
        NotFound,
        UserEmailNotFound,
    }
}