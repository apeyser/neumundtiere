use crate::rpn::*;
use regex::{Regex, Captures};
use std::str::FromStr;

pub struct Reader<'h> {
    string: &'h str,
    regex: Regex,
}

#[derive(Debug, Clone)]
pub enum Error {
    IntParse(<i64 as FromStr>::Err),
    IllegalSym(String),
    Illformed(String),
}

static REGEX: &str = r"^\s*(?:(?<int>\d+)|(?<name>\w+)|(?<illegal>\S+))\s*";

impl<'h> Reader<'h> {
    pub fn new(string: &'h str) -> Self {
        match Regex::new(REGEX) {
            Err(err) => panic!("Failed regex: {:?}", err),
            Ok(regex) => Reader {string, regex},
        }
    }

    pub fn parsed(self) -> Result<Vec<Frame>, Error> {
        let mut vec = Vec::<Frame>::new();
        let mut string = self.string;
        while string.len() > 0 {
            let Some(captures) = self.regex.captures(string) else {
                return Err(Error::Illformed(String::from(string)));
            };
            let Some(m) = captures.get(0) else {
                panic!("Bad capture: {:?}", captures);
            };
            string = &string[m.end()..];
            match Self::convert(captures) {
                Err(e) => return Err(e),
                Ok(frame) => vec.push(frame),
            };
        };
        
        Ok(vec)
    }

    pub fn parse(string: &'h str) -> Result<Vec<Frame>, Error> {
        Self::new(string).parsed()
    }

    pub fn result(string: &'h str) -> Option<Vec<Frame>> {
        match Reader::parse(string) {
            Err(e) => {
                println!("Read failure {:?}", e);
                None
            },
            Ok(frames) => Some(frames),
        }
    }

    fn convert(captures: Captures) -> Result<Frame, Error> {
        if let Some(m) = captures.name("int") {
            match m.as_str().parse::<i64>() {
                Ok(i) => Ok(i.into()),
                Err(e) => Err(Error::IntParse(e)),
            }
        } else if let Some(m) = captures.name("name") {
            Ok(String::from(m.as_str()).into())
        } else if let Some(m) = captures.name("illegal") {
            Err(Error::IllegalSym(m.as_str().into()))
        } else {
            panic!("Illegal match {:?}", captures);
        }
    }
}
