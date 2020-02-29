use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ServerError{
	InvalidFmt,
	InvalidArgs,
	PortParse,
	PortRange,
}

impl fmt::Display for ServerError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
		let err_str = match self{
			ServerError::InvalidFmt => "Invalid argument format",
			ServerError::InvalidArgs => "Invalid arguments",
			ServerError::PortParse => "Failed to parse port",
			ServerError::PortRange => "Port outside of valid range"
		};
		write!(f, "{}", err_str)
	}
}

impl error::Error for ServerError{
}
