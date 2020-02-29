mod server_error;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::error::Error;
use std::env;
use std::fs;
use std::collections::HashMap;
use server_error::ServerError;

pub struct Server{
	listener: TcpListener,
	aliases: HashMap<String, String>,
	root: String,

	pub addr: SocketAddr
}

impl Server{
	pub fn new(mut args: env::Args) -> Result<Server, Box<dyn Error>>{
		let mut root = String::new();
		let mut port = String::from("7878");
		let mut aliases = HashMap::new();

		let program_name = args.next().unwrap();
		for arg in args{
			let arg: Vec<_> = (&arg[2..]).splitn(2, '=').collect();
			//if length is 1 then there was no =
			if arg.len() == 1{
				return Err(Box::new(ServerError::InvalidFmt));
			};
			let (arg, val) = (arg[0], arg[1]);

			match arg{
				"root" => root = String::from(val),
				"port" => {
					//check if the port value passed is valid
					let tmp: i32 = match val.parse(){
						Ok(v) => v,
						Err(_) => return Err(Box::new(ServerError::PortParse))
					};

					if tmp >= u16::min_value() as i32 && tmp <= u16::max_value() as i32{
						port = String::from(val);
					}
					else{
						return Err(Box::new(ServerError::PortRange))
					}
				},
				"alias" => {
					let mut alias_list = val.split(',');
					let name = String::from(alias_list.next().unwrap());
					for alias in alias_list{
						aliases.insert(String::from(alias), name.clone());
					}
				}
				"help" => {
					println!(
						"usage: {} <flags>.\n
						flags:\n
						\t--root=<path>\n
						\t\tspecify the root path of the website, the default is the current working directory\n\n
						\t--port=<0-65535>\n
						\t\tspecify what port to listen to connections on, default is 7878\n\n
						\t--alias=<comma separated list>\n
						\t\tspecifies and alias to the first item in the list\n
						\t\te.g. `--alias=index.html,index` the path /index will now refer to index.html\n\n",
						program_name);
				}
				_ => return Err(Box::new(ServerError::InvalidArgs))
			}
		}

		let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
		let addr = listener.local_addr()?;
		if root.is_empty(){
			root.push_str("/var/www/html/");
		};

		Ok(Server{
			listener,
			aliases,
			root,
			addr
		})
	}

	pub fn run(&mut self){
		for stream in self.listener.incoming(){
			let stream = match stream{
				Ok(s) => s,
				Err(e) => {
					println!("Error accepting connection: {}", e);
					continue;
				}
			};

			self.handle_connection(stream);
		};
	}

	fn handle_connection(&self, mut stream: TcpStream){
		let mut buffer = [0; 1024];
		if let Err(e) = stream.read(&mut buffer){
			println!("Error reading request: {}", e);
			return;
		};

		let request = String::from_utf8_lossy(&buffer); 
		println!("Request: {}", request);
		let request = match Server::uri_from_request(&request){
			Some(s) => {
				if s == "/"{
					String::from(s)+"index.html"
				}
				else{
					String::from(s)
				}
			},
			None => return
		};

		self.send_file(&stream, &request);
	}

	fn send_file(&self, stream: &TcpStream, request: &str){
		static ERR404: ErrorCode = ErrorCode{
			response: Response{
				code: "404",
				phrase: "NOT_FOUND",
				message: "Error: 404 not found"
			},
			default_search: "404.html"
		};

		//store message content here so that references can live long enough
		let mut message = String::new();
		let response = if let Ok(s) = fs::read_to_string(self.root.clone()+request){
			message = s;
			Response::new("200", "OK", &message)
		}
		else{
			//all of our requests begin with / which messes up the search
			if let Some(s) = self.aliases.get(&request[1..]){
				match fs::read_to_string(self.root.clone()+s){
					Ok(c) => {
						message = c;
					},
					Err(e) => {
						println!("Failed to read {}: {}", request, e);
					}
				};
			};

			if !message.is_empty(){
				Response::new("200", "OK", &message)
			}
			else{
				if let Ok(c) = fs::read_to_string(self.root.clone()){
					message = c;
					Response::new(ERR404.response.code, ERR404.response.phrase, &message)
				}
				else{
					ERR404.response.clone()
				}
			}
		};

		Server::respond(stream, &response);
	}

	fn respond(mut stream: &TcpStream, Response{code, phrase, message}: &Response){
		let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", code, phrase, message);

		if let Err(e) = stream.write(response.as_bytes()){
			println!("Error sending response: {}", e);
			return;
		};

		if let Err(e) = stream.flush(){
			println!("Error sending resposne: {}", e);
			return
		};
	}

	fn uri_from_request<'a>(request: &str) -> Option<&str>{
		//first line is always in the format of "GET URI HTTP/VER"
		request.splitn(3, ' ').skip(1).next()
	}
}

struct Response<'a>{
	pub code: &'a str,
	pub phrase: &'a str,
	pub message: &'a str
}

impl<'a> Response<'a>{
	pub fn new(code: &'a str, phrase: &'a str, message: &'a str) -> Response<'a>{
		Response{
			code,
			phrase,
			message
		}
	}
}

impl<'a> Clone for Response<'a>{
	fn clone(&self) -> Response<'a>{
		Response{
			code: self.code,
			phrase: self.phrase,
			message: self.message
		}
	}
}

struct ErrorCode<'a>{
	pub response: Response<'a>,
	pub default_search: &'a str
}
