use std::fs;
use std::path::Path;
use std::net::TcpStream;

use mysql::prelude::*;
use mysql::*;
use native_tls::{TlsConnector, TlsStream};
use imap;
use imap_proto::types::BodyStructure;
use regex::Regex;
use toml;

mod config;

#[derive(Debug, Eq, PartialEq)]
enum ReportFileType {
	Zip,
	Gzip
}

/// Searches the given IMAP BodyStructure for a report file.
fn find_report(body_structure: &imap_proto::types::BodyStructure, prefix: String) -> Option<(String, ReportFileType)> {
	match body_structure {
		BodyStructure::Multipart { common: _, bodies, extension: _ } => {
			// unwrap the multipart message
			for (i, body) in bodies.iter().enumerate() {
				let mut actual_prefix = prefix.clone();
				if actual_prefix != "" {
					actual_prefix += ".";
				}
				let result = find_report(body, actual_prefix + &i.to_string());
				if result.is_some() {
					return result;
				}
			}

			None
		},

		BodyStructure::Basic { common, other: _, extension: _ } => {
			// this might be our attachment
			// check if it's a filetype we know about
			// println!("{:#?}", common);
			println!("{:#?}", common.ty);
			let filetype_option = match (
				common.ty.ty.to_lowercase().as_str(),
				common.ty.subtype.to_lowercase().as_str()
			) {
				("application", "gzip") => Some(ReportFileType::Gzip),
				("application", "zip") => Some(ReportFileType::Zip),
				_ => None,
			};

			let part_number = match prefix.as_str() {
				"" => "1".to_string(),
				_ => prefix,
			};

			match filetype_option {
				Some(filetype) => Some((part_number, filetype)),
				None => None,
			}
		},

		BodyStructure::Message { common: _, other: _, envelope: _, body: _, lines: _, extension: _, } => {
			// we shouldn't get this
			// ignore it

			None
		},
		BodyStructure::Text { common: _, other: _, lines: _, extension: _ } => {
			// don't care about text, it's probably just some human-readable message
			// ignore it

			None
		},
	}
}

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

		if let Some(captures) = re.captures(&subject_text) {
			let report_id = captures.get(1).unwrap().as_str();

			if let Some(bodystructure) = fetch_result.bodystructure() {
				let report_info = find_report(bodystructure, "".to_string());
				if let Some((part_number, report_type)) = report_info {
					println!("{} {:?}", part_number, report_type);
				} else {
					println!("Couldn't find report file in message for ID {}", report_id);
				}
			} else {
				println!("Skipping report ID {}", report_id);
			}
		} else {
			println!("Skipping subject line '{}'", subject_text);
		}
	}

	dmarc_session.logout().unwrap();
}
