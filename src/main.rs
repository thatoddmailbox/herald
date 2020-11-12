use std::fs;
use std::path::Path;
use std::net::TcpStream;
use std::io::{prelude::*, Cursor};

use base64;
use mysql::prelude::*;
use mysql::*;
use native_tls::{TlsConnector, TlsStream};
use imap;
use imap_proto::types::SectionPath;
use regex::Regex;
use toml;
use zip;

mod config;
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

		if let Some(captures) = re.captures(&subject_text) {
			let report_id = captures.get(1).unwrap().as_str();

			if let Some(bodystructure) = fetch_result.bodystructure() {
				let report_info = message::find_report(bodystructure, "".to_string());
				if let Some((part_number, report_type)) = report_info {
					let section = format!("BODY[{}]", part_number);

					if report_type != types::ReportFileType::Zip {
						panic!("Non-zip reports not supported yet!");
					}

					let message_results = dmarc_session.fetch(&fetch_result.message.to_string(), section).unwrap();
					let message	= message_results.first().unwrap();

					let body_data = message.section(&SectionPath::Part(
						[ 1 ].to_vec(), None
					)).unwrap();
					let body_text = std::str::from_utf8(body_data).unwrap().to_owned();
					let body_text_no_lines = body_text.replace("\r", "").replace("\n", "");

					if let Ok(decoded_data) = base64::decode(body_text_no_lines.as_bytes()) {
						let body_reader = Cursor::new(decoded_data);

						let mut archive = zip::ZipArchive::new(body_reader).unwrap();

						if archive.len() != 1 {
							println!("Couldn't find report file in message for ID {}", report_id);
							continue;
						}

						let report_file = archive.by_index(0).unwrap();
						println!("{}", report_file.name());
						let file_bytes = report_file.bytes().map(|x| x.unwrap()).collect::<Vec<_>>();
						println!("{}", String::from_utf8(file_bytes).unwrap());
					} else {
						println!("Couldn't decode base64 in message for ID {}", report_id);
					}
					return;
				} else {
					println!("Couldn't find report archive in message for ID {}", report_id);
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
