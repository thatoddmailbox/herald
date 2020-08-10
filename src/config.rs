use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config<'a> {
	#[serde(borrow)]
	pub database: DatabaseConfig<'a>,

	#[serde(borrow)]
	pub dmarc: IMAPConfig<'a>,

	#[serde(borrow)]
	pub tlsrpt: IMAPConfig<'a>
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseConfig<'a> {
	pub host: &'a str,
	pub username: &'a str,
	pub password: &'a str,
	pub database: &'a str
}

#[derive(Deserialize, Serialize)]
pub struct IMAPConfig<'a> {
	pub enabled: bool,
	pub host: &'a str,
	pub port: u16,
	pub tls: bool,
	pub username: &'a str,
	pub password: &'a str,
	pub folder: &'a str
}

pub const DEFAULT: Config = Config{
	database: DatabaseConfig{
		host: "localhost",
		username: "username",
		password: "password123",
		database: "herald"
	},
	dmarc: IMAPConfig{
		enabled: true,
		host: "localhost",
		port: 993,
		tls: true,
		username: "reports@dmarc.some-cool-address.invalid",
		password: "password123",
		folder: "INBOX"
	},
	tlsrpt: IMAPConfig{
		enabled: true,
		host: "localhost",
		port: 993,
		tls: true,
		username: "reports@tlsrpt.some-cool-address.invalid",
		password: "password123",
		folder: "INBOX"
	}
};