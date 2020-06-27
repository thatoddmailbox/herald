use std::fs;
use std::path::Path;

use toml;

mod config;

fn main() {
	// fs::File::exi
	let config_path = Path::new("config.toml");
	if !config_path.exists() {
		println!("Could not find config, creating default...");
		std::fs::write(config_path, toml::to_string(&config::DEFAULT).unwrap())
			.expect("Could not write config file");
	}

	let config_text = fs::read_to_string(config_path).expect("Could not read config file");

	let config: config::Config = toml::from_str(&config_text).unwrap();
	// println!("{:#?}", config);
}
