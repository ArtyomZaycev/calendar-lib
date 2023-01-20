pub mod load {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	use chrono::NaiveDateTime;
	

	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {
		pub id: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub id: i32,
		pub user_id: i32,
		pub name: String,
		pub description: Option<String>,
		pub start: NaiveDateTime,
		pub end: NaiveDateTime,
		pub access_level: i32,
	}
}

pub mod load_array {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};


	pub static METHOD: Method = Method::Get;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	use super::load::Response as ResponseSingle;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {
		pub array: Vec<ResponseSingle>
	}
}

pub mod insert {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	use chrono::NaiveDateTime;
	

	pub static METHOD: Method = Method::Post;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {
		pub name: String,
		pub description: Option<String>,
		pub start: NaiveDateTime,
		pub end: NaiveDateTime,
		pub access_level: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod update {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	use chrono::NaiveDateTime;
	

	pub static METHOD: Method = Method::Patch;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {
		pub id: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {
		pub user_id: Option<i32>,
		pub name: Option<String>,
		pub description: Option<Option<String>>,
		pub start: Option<NaiveDateTime>,
		pub end: Option<NaiveDateTime>,
		pub access_level: Option<i32>,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {}
}


pub mod delete {
	use crate::api_types::utils::{Method, AuthRequirement};
	use serde::{Serialize, Deserialize};
	

	pub static METHOD: Method = Method::Delete;
	pub static AUTH: AuthRequirement = AuthRequirement::Any;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Args {
		pub id: i32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Body {}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Response {}
}