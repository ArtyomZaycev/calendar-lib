pub mod load {
	use crate::api::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};


	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::Admin;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {
		pub email: String,
		pub password: String
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub user_id: i32,
		pub access_level: i32,
		pub key: Vec<u8>,
	}
}

pub mod load_array {
	use crate::api::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};


	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::SuperAdmin;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {
		pub email: String,
		pub password: String
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub user_id: i32,
		pub access_level: i32,
		pub key: Vec<u8>,
	}
}