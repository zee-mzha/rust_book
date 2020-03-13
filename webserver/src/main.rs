mod server;

use std::{
	env,
	sync::Arc
};
use server::Server;
use server::ServerError;

fn main(){
	let server = match Server::new(env::args()){
		Ok(s) => Arc::new(s),
		Err(e) => {
			match e{
				ServerError::HelpRequest => {},
				_ => eprintln!("Error occured when configuring server: {}", e)
			};
			return;
		}
	};

	if let Err(e) = Server::run(server.clone()){
		eprintln!("Error occured when running server: {}", e);
	}
}
