use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum CTErrors{
    CLI,
    Ports
}

impl Display for CTErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CLI => write!(f, "CLI Error"),
            Self::Ports => write!(f, "Ports error")
        }
    }
}

impl Error for CTErrors {}