use std::io::{self, Write};

pub fn get_trimmed_input() -> String{
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.expect("failed to read input line");
	input.trim().to_owned()
}

pub fn flush_stdout(){
	io::stdout().flush().unwrap();
}

pub fn wait_for_enter(){
	print!("press enter to continue: ");
	flush_stdout();

	let mut tmp = String::new();
	io::stdin().read_line(&mut tmp)
		.expect("failed to read input line");
}
