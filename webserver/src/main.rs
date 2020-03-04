mod server;

use std::env;
use server::Server;
use server::ServerError;

fn main(){
	let server = match Server::new(env::args()){
		Ok(s) => s,
		Err(e) => {
			match e{
				ServerError::HelpRequest => {},
				_ => println!("Error configuring server: {}", e)
			}
			return;
		}
	};
	server.run().unwrap();
}
