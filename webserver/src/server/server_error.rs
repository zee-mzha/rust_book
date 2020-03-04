use std::{
	fmt,
	error
};

#[derive(Debug)]
pub enum ServerError{
	InvalidFmt,
	InvalidArgs,
	PortParse,
	PortRange,
	ThreadCountParse,
	ZeroThreadCount,
	HelpRequest
}

impl fmt::Display for ServerError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
		let err_str = match self{
			ServerError::InvalidFmt => "Invalid argument format",
			ServerError::InvalidArgs => "Invalid arguments",
			ServerError::PortParse => "Failed to parse port",
			ServerError::PortRange => "Port outside of valid range",
			ServerError::ThreadCountParse => "Failed to parse thread count",
			ServerError::ZeroThreadCount => "A thread count of zero is not allowed",
			ServerError::HelpRequest => "Help flag passed this is not an error, this case should be handled explicitly"
		};
		write!(f, "{}", err_str)
	}
}

impl error::Error for ServerError{
}
