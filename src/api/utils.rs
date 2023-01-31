pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

pub enum AuthRequirement {
    No,
    Any,
    Full,
    Admin,
    SuperAdmin,
}