use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
	pub files: Vec<String>,
	pub search_strings: Vec<String>
}

impl Config{
	pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
		args.next();

		//get search strings
		let search_strings = match args.next(){
			Some(s) => s.split("\\|").map(|s| String::from(s)).collect(),
			None => return Err("Didn't get a query string")
		};

		//get file paths
		let files = args.collect();

		Ok(Config{
			files,
			search_strings
		})
	}

	pub fn run(&self) -> Result<(), Box<dyn Error>>{
		let print_file = self.files.len() > 1;
		for file in &self.files{
			let content = fs::read_to_string(file)?;
			let matches: Vec<&str> = content.lines().filter(|line|{
				for needle in &self.search_strings{
					if line.contains(needle) {return true;}
				}
				return false;
			}).collect();

			if print_file{
				for m in matches{
					println!("{}:{}", file, m.trim());
				}
			}
			else{
				for m in matches{
					println!("{}", m.trim());
				}
			}
		}

		Ok(())
	}
}
