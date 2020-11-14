use std::fs;
use std::path::Path;
use std::net::TcpStream;

use base64;
use mysql::prelude::*;
use mysql::*;
use native_tls::{TlsConnector, TlsStream};
use imap;
use imap_proto::types::SectionPath;
use regex::Regex;
use serde_xml_rs::from_reader;
use toml;

mod config;
mod dmarc;
mod message;
mod types;

fn main() {
	let re = Regex::new(r"Report-ID: (.*)").unwrap();

	/*
	 * config parsing
	 */
	let config_path = Path::new("config.toml");
	if !config_path.exists() {
		println!("Could not find config, creating default...");
		std::fs::write(config_path, toml::to_string(&config::DEFAULT).unwrap())
			.expect("Could not write config file");
	}

	let config_text = fs::read_to_string(config_path).expect("Could not read config file");
	let config: config::Config = toml::from_str(&config_text).unwrap();

	/*
	 * database connection
	 */
	// let pool = Pool::new(format!(
	// 	"mysql://{}:{}@{}/{}",
	// 	config.database.username, config.database.password, config.database.host, config.database.database
	// )).unwrap();
	// let mut db_conn = pool.get_conn().unwrap();

	/*
	 * imap connection
	 */

	let tls = TlsConnector::builder().build().unwrap();
	let client = imap::connect((config.dmarc.host, config.dmarc.port), config.dmarc.host, &tls).unwrap();
	let mut dmarc_session: imap::Session<TlsStream<TcpStream>> = client.login(config.dmarc.username, config.dmarc.password).map_err(|e| e.0).unwrap();

	let mailbox = dmarc_session.select("INBOX").unwrap();
	// println!("{:#?}", mailbox);

	let fetch_results = dmarc_session.fetch("1:90", "ALL BODYSTRUCTURE").unwrap();

	for fetch_result in fetch_results.iter() {
		let envelope: &imap_proto::types::Envelope = fetch_result.envelope().unwrap();
		let subject_text = String::from_utf8_lossy(envelope.subject.unwrap_or(b""));

		// parse the report ID out of the subject
		let captures = if let Some(x) = re.captures(&subject_text) { x } else {
			println!("Skipping subject line '{}'", subject_text);
			continue;
		};
		let report_id = captures.get(1).unwrap().as_str();

		// try to get the bodystructure
		let bodystructure = if let Some(x) = fetch_result.bodystructure() { x } else {
			println!("Skipping report ID {}", report_id);
			continue;
		};

		// find the report
		let report_info = message::find_report(bodystructure, "".to_string());
		let (part_number, report_type) = if let Some(x) = report_info { x } else {
			println!("Couldn't find report archive in message for ID {}", report_id);
			continue;
		};

		// try to get the right part from the message
		let section = format!("BODY[{}]", part_number);
		let message_results = dmarc_session.fetch(&fetch_result.message.to_string(), section).unwrap();
		let message	= message_results.first().unwrap();

		let parsed_path = part_number.split(".").map(|x| {
			x.parse::<u32>().unwrap()
		}).collect::<Vec<u32>>();

		// get the body text
		let body_data = message.section(&SectionPath::Part(
			parsed_path, None
		)).unwrap();
		let body_text = std::str::from_utf8(body_data).unwrap().to_owned();
		let body_text_no_lines = body_text.replace("\r", "").replace("\n", "");

		// parse it as base64
		let decoded_data = if let Ok(x) = base64::decode(body_text_no_lines.as_bytes()) { x } else {
			println!("Couldn't decode base64 in message for ID {}", report_id);
			continue;
		};

		// extract it
		let file_bytes = message::read_report(report_type, decoded_data).unwrap();
		let report: dmarc::types::Report = from_reader(file_bytes.as_bytes()).unwrap();
		println!("{:#?}", report);
	}

	dmarc_session.logout().unwrap();
}
