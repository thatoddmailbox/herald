use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config<'a> {
	#[serde(borrow)]
	database: DatabaseConfig<'a>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DatabaseConfig<'a> {
	host: &'a str,
	username: &'a str,
	password: &'a str
}

pub const DEFAULT: Config = Config{
	database: DatabaseConfig{
		host: "localhost",
		username: "username",
		password: "password123"
	}
};