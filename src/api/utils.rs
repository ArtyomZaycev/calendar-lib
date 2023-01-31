#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AuthRequirement {
    No,
    Any,
    Full,
    Admin,
    SuperAdmin,
}