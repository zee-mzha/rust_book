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
			"Invalid argument format",
			"Invalid arguments",
			"Failed to parse port",
			"Port outside of valid range"
		};
		write!(f, err_str);
		match self{
			ServerError::InvalidFmt => write!(f, "Invalid argument format"),
			ServerError::InvalidArgs => write!(f, "Invalid arguments"),
			ServerError::PortParse => write!(f, "Failed to parse port"),
			ServerError::PortRange => write!(f, "Port outside of valid range"),
		}
	}
}

impl error::Error for ServerError{
}
