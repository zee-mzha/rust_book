mod config;

use std::env;
use std::process;
use config::Config;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|e|{
		eprintln!("Failed to parse arguments: {}", e);
		process::exit(1);
	});

	config.run().unwrap_or_else(|e|{
		eprintln!("Failed to search file: {}", e);
		process::exit(1);
	});
}
