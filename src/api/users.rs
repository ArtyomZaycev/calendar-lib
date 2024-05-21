pub mod types {}

pub mod load_array {
    use crate::api::utils::{LoadArrayQuery, User};
    use http::Method;

    pub const METHOD: Method = Method::GET;
    pub const PATH: &str = "users";

    pub type Args = LoadArrayQuery;
    pub type Response = Vec<User>;
}
