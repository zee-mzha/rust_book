mod server_error;

use std::{
	env,
	fs,
	collections::HashMap,
	sync::Arc
};
use tokio::{
	prelude::*,
	net::{TcpListener, TcpStream},
	sync::mpsc,
	runtime::Runtime
};
use async_std::io;
pub use server_error::ServerError;

pub struct Server{
	port: String,
	aliases: HashMap<String, String>,
	root: String,
}

impl Server{
	pub fn new(mut args: env::Args) -> Result<Self, ServerError>{
		let mut root = String::new();
		let mut port = String::from("7878");
		let mut aliases = HashMap::new();

		let program_name = args.next().unwrap();
		for arg in args{
			let arg: Vec<_> = (&arg[2..]).splitn(2, '=').collect();
			
			let (opt, val): (&str, &str);
			if arg.len() == 1{
				if arg[0] == "help"{
					opt = arg[0];
					//get the rust compiler to shut up about val being possibly unintialized
					val = "";
				}
				else{
					return Err(ServerError::InvalidArgs);
				}
			}
			else{
				opt = arg[0];
				val = arg[1];
			}

			match opt{
				"root" => root = String::from(val),
				"port" => {
					//check if the port value passed is valid
					let tmp: i32 = match val.parse(){
						Ok(v) => v,
						Err(_) => return Err(ServerError::PortParse)
					};

					if tmp >= u16::min_value() as i32 && tmp <= u16::max_value() as i32{
						port = String::from(val);
					}
					else{
						return Err(ServerError::PortRange)
					}
				},
				"alias" => {
					let mut alias_list = val.split(',');
					let name = String::from(alias_list.next().unwrap());
					for alias in alias_list{
						aliases.insert(String::from(alias), name.clone());
					}
				},
				"help" => {
					println!(
						"usage: {} <flags>.\n\
						flags:\n\
						\t--root=<path>\n\
						\t\tspecify the root path of the website, the default is the current working directory\n\n\
						\t--port=<0-65535>\n\
						\t\tspecify what port to listen to connections on, default is 7878\n\n\
						\t--alias=<comma separated list>\n\
						\t\tspecifies and alias to the first item in the list\n\
						\t\te.g. `--alias=index.html,index` the path /index will now refer to index.html\n\n",
						program_name);
						return Err(ServerError::HelpRequest);
				},
				_ => return Err(ServerError::InvalidArgs)
			}
		}

		if root.is_empty(){
			root.push_str("/var/www/html/");
		};

		Ok(Server{
			aliases,
			root,
			port
		})
	}

	pub fn run(arc_self: Arc<Self>) -> Result<(), std::io::Error>{
		let mut runtime = Runtime::new()?;
		runtime.block_on(async{
			let mut listener = TcpListener::bind(format!("127.0.0.1:{}", arc_self.port)).await?;
			let (mut tx, mut rx) = mpsc::channel::<()>(1);

			let listen = async move{
				loop{
					tokio::select! {
						_ = rx.recv() => {
							break;
						}
						stream = listener.accept() => {
							let stream = match stream{
								Ok((s, _)) => s,
								Err(e) => {
									eprintln!("Error accepting connection: {}", e);
									continue;
								}
							};

							tokio::spawn(Server::handle_connection(arc_self.clone(), stream));
						}
					};
				};
			};

			let cli = async move{
				let mut input = String::new();
				loop{
					if let Err(_) = io::stdin().read_line(&mut input).await{
						eprintln!("Failed to read input, terminating server");
						tx.send(()).await.unwrap();
						break;
					}
					input = input.trim().to_lowercase();

					if input == "quit"{
						//receiver will never drop before transmitter
						tx.send(()).await.unwrap();
						break;
					}
				}

				Ok::<(), io::Error>(())
			};

			let _ = tokio::join!(listen, cli);
			Ok::<(), std::io::Error>(())
		})
	}

	async fn handle_connection(arc_self: Arc<Self>, mut stream: TcpStream){
		let mut buffer = [0; 1024];
		if let Err(e) = stream.read(&mut buffer).await{
			eprintln!("Error reading request: {}", e);
			return;
		};

		let request = String::from_utf8_lossy(&buffer); 
		let request = match Server::uri_from_request(&request){
			Some(s) => {
				if s == "/"{
					"/index.html"
				}
				else{
					s
				}
			},
			None => return
		};

		Server::send_file(&*arc_self, &mut stream, request).await;
	}

	async fn send_file(&self, stream: &mut TcpStream, request: &str){
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
						eprintln!("Failed to read {}: {}", request, e);
					}
				};
			};

			if !message.is_empty(){
				Response::new("200", "OK", &message)
			}
			else{
				if let Ok(c) = fs::read_to_string(&self.root){
					message = c;
					Response::new(ERR404.response.code, ERR404.response.phrase, &message)
				}
				else{
					ERR404.response.clone()
				}
			}
		};

		Server::respond(stream, &response).await;
	}

	async fn respond<'a>(stream: &mut TcpStream, Response{code, phrase, message}: &Response<'a>){
		let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", code, phrase, message);

		if let Err(e) = stream.write(response.as_bytes()).await{
			eprintln!("Error sending response: {}", e);
			return;
		};

		if let Err(e) = stream.flush().await{
			eprintln!("Error sending resposne: {}", e);
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
