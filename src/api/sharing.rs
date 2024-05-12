use super::auth::types::AccessLevel;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SharedTablePermission {
    pub view: bool,
    pub edit: bool,
    pub create: bool,
    pub delete: bool,
}

impl SharedTablePermission {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SharedPermissions {
    pub access_level: i32,
    pub access_levels: SharedTablePermission,
    pub events: SharedTablePermission,
    pub event_templates: SharedTablePermission,
    pub schedules: SharedTablePermission,
    pub allow_share: bool,
}

impl SharedPermissions {
    pub const FULL: Self = Self {
        access_level: AccessLevel::MAX_LEVEL,
        access_levels: SharedTablePermission::FULL,
        events: SharedTablePermission::FULL,
        event_templates: SharedTablePermission::FULL,
        schedules: SharedTablePermission::FULL,
        allow_share: true,
    };
    
    pub const NONE: Self = Self {
        access_level: AccessLevel::MIN_LEVEL - 1,
        access_levels: SharedTablePermission::NONE,
        events: SharedTablePermission::NONE,
        event_templates: SharedTablePermission::NONE,
        schedules: SharedTablePermission::NONE,
        allow_share: false,
    };

    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
}