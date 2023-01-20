pub mod load { // NOT IMPLEMENTED
	use serde::{Serialize, Deserialize};


	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub id: i32,
		pub name: String,
		pub description: Option<String>,
	}
}

pub mod load_array {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	

	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::No; // Any ?

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	use super::load::Response as ResponseSingle;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub array: Vec<ResponseSingle>,
	}
}

pub mod load_user_roles {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	

	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {
		pub user_id: Option<i32>, // Admin can load roles of any user
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	use super::load::Response as ResponseSingle;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub array: Vec<ResponseSingle>,
	}
}

pub mod insert {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	

	pub static METHOD: Method = Method::Patch;
	pub static AUTH: AuthRequirement = AuthRequirement::SuperAdmin;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {
		pub user_id: i32,
		pub role_id: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod delete {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	

	pub static METHOD: Method = Method::Delete;
	pub static AUTH: AuthRequirement = AuthRequirement::SuperAdmin;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {
		pub id: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {}
}