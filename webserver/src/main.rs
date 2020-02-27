mod server;
use server::Server;
use std::env;

fn main(){
	let mut server = Server::new(env::args()).unwrap();
	server.run();
}
