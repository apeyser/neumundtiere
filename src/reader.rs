use std::fmt::{self, Display, Formatter};
use std::error;

use super::rpn::{self, *};
use regex::{Regex, Captures};
use std::str::FromStr;

pub struct Reader<'a> {
    regex: Regex,
    vm: &'a mut Vm,
}

#[derive(Debug, Clone)]
pub enum Error {
    IntParse(<i64 as FromStr>::Err),
    IllegalSym(String),
    Illformed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::IntParse(err) => write!(f, "Int parsing error: {err}"),
            Error::IllegalSym(string) => write!(f, "Illegal symbol: {string}"),
            Error::Illformed(string) => write!(f, "Illformed string: {string}"),
        }
    }
}

impl error::Error for Error {}

static REGEX: &str = r"^\s*(?:(?<int>\d+)|(?<name>[^\s{}\[\]]+)|(?<illegal>\S+))\s*";

impl<'a> Reader<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        match Regex::new(REGEX) {
            Err(err) => panic!("Failed regex: {:?}", err),
            Ok(regex) => Reader {regex, vm},
        }
    }

    pub fn parse(&mut self, string: String) -> Result<Vec<Frame>, Error> {
        let mut vec = Vec::<Frame>::new();
        let mut string = string.as_str();
        while string.len() > 0 {
            let Some(captures) = self.regex.captures(string) else {
                return Err(Error::Illformed(String::from(string)))
            };
            let Some(m) = captures.get(0) else {
                panic!("Bad capture: {:?}", captures)
            };
            string = &string[m.end()..];
            vec.push(self.convert(captures)?);
        };
        
        Ok(vec)
    }

    pub fn exec(&mut self, frames: Vec<Frame>) -> Result<Option<Frame>, rpn::Error> {
        self.vm.exec(frames)
    }

    #[allow(dead_code)]
    pub fn result(&mut self, string: String) -> Option<Vec<Frame>> {
        match self.parse(string) {
            Err(e) => {
                println!("Read failure {e:?}");
                None
            },
            Ok(frames) => Some(frames),
        }
    }

    fn convert(&mut self, captures: Captures) -> Result<Frame, Error> {
        if let Some(m) = captures.name("int") {
            match m.as_str().parse::<i64>() {
                Ok(i) => Ok(Frame::Num(i.into())),
                Err(e) => Err(Error::IntParse(e)),
            }
        } else if let Some(m) = captures.name("name") {
            Ok(self.vm.intern(String::from(m.as_str())))
        } else if let Some(m) = captures.name("illegal") {
            Err(Error::IllegalSym(m.as_str().into()))
        } else {
            panic!("Illegal match {:?}", captures);
        }
    }
}
