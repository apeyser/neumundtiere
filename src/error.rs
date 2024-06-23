use std::fmt::{self, Display, Formatter};
use std::error;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Error {
    IntParse(<i64 as FromStr>::Err),
    IllegalSym(String),
    Illformed(String),
    Quit,
    StackUnderflow,
    OpType,
    Unknown(String),
}

impl<T> Into<Result<T, Error>> for Error {
    fn into(self) -> Result<T, Error> {
        Err(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::Quit => write!(f, "Quitting"),
            Error::StackUnderflow => write!(f, "Stack underflow"),
            Error::OpType => write!(f, "Illegal operand type"),
            Error::Unknown(string) => write!(f, "Unknown name {string}"),
            Error::IntParse(err) => write!(f, "Int parsing error: {err}"),
            Error::IllegalSym(string) => write!(f, "Illegal symbol: {string}"),
            Error::Illformed(string) => write!(f, "Illformed string: {string}"),
        }
    }
}

impl error::Error for Error {}
