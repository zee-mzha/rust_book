mod server;

use std::{
	env,
	process
};
use server::Server;

fn main(){
	let mut server = match Server::new(env::args()){
		Ok(s) => s,
		Err(e) => {
			println!("Failed to start server: {}", e);
			process::exit(1);
		}
	};
	server.run().unwrap();
}
