use std::{
	fmt,
	error
};

#[derive(Debug)]
pub enum ServerError{
	InvalidArgs,
	PortParse,
	PortRange,
	HelpRequest
}

impl fmt::Display for ServerError{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let err_str = match self{
			ServerError::InvalidArgs => "Invalid arguments",
			ServerError::PortParse => "Failed to parse port",
			ServerError::PortRange => "Port outside of valid range",
			ServerError::HelpRequest => "Help flag passed this is not an error, this case should be handled explicitly"
		};
		write!(f, "{}", err_str)
	}
}

impl error::Error for ServerError{
}
