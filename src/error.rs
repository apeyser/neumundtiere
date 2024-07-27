use std::fmt::{self, Display, Formatter};
use std::error;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Error {
    IntParse(<i64 as FromStr>::Err, String),
    FloatParse(<f64 as FromStr>::Err, String),
    USizeParse(<usize as FromStr>::Err, String),
    IllegalSym(String),
    Illformed(String),
    Quit,
    StackUnderflow,
    OpType,
    Unknown(String),
    Range {len: usize, index: usize},
    Dropped,
    IllNeg,
    MissingKey(String)
}

impl<T> Into<Result<T, Error>> for Error {
    fn into(self) -> Result<T, Error> {
        Err(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::Quit               => write!(f, "Quitting"),
            Error::StackUnderflow     => write!(f, "Stack underflow"),
            Error::OpType             => write!(f, "Illegal operand type"),
            Error::Unknown(string)    => write!(f, "Unknown name {string}"),
            Error::IntParse(err, s)   => write!(f, "Int parsing error: {err} ({s})"),
            Error::FloatParse(err, s) => write!(f, "Float parsing error: {err} ({s})"),
            Error::IllegalSym(string) => write!(f, "Illegal symbol: {string}"),
            Error::Illformed(string)  => write!(f, "Illformed string: {string}"),
            Error::Range{len, index}  => write!(f, "Illegal range: len({len}), index({index})"),
            Error::IllNeg             => write!(f, "Illegal negative in range"),
            Error::Dropped            => write!(f, "Illegal reference to dropped object"),
            Error::MissingKey(s)      => write!(f, "Missing key {s} in Dict"),
        }
    }
}

impl error::Error for Error {}
