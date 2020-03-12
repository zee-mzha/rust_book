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
				_ => println!("Error configuring server: {}", e)
			}
			return Ok(());
		}
	};
	Server::run(server).await;

	Ok(())
}
