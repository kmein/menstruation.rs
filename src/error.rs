use ansi_term::Colour::Red;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Net(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let error_style = Red.bold();
        match self {
            Error::Parse(message) => write!(f, "{} {}", error_style.paint("PARSE ERROR"), message),
            Error::Net(message) => write!(f, "{} {}", error_style.paint("NETWORK ERROR"), message),
        }
    }
}
