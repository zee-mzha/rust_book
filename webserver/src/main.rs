mod server;

use std::{
	env,
	sync::Arc
};
use server::Server;
use server::ServerError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	let server = match Server::new(env::args()){
		Ok(s) => Arc::new(s),
		Err(e) => {
			match e{
				ServerError::HelpRequest => {},
				_ => eprintln!("Error occured when configuring server: {}", e)
			};
			return Ok(());
		}
	};
	if let Err(e) = Server::run(server).await{
		eprintln!("Error occured when running server: {}", e);
	}

	Ok(())
}
