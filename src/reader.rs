use crate::rpn::*;
use regex::{Regex, Captures};
use std::str::FromStr;

pub struct Reader {
    regex: Regex,
}

#[derive(Debug, Clone)]
pub enum Error {
    IntParse(<i64 as FromStr>::Err),
    IllegalSym(String),
    Illformed(String),
}

static REGEX: &str = r"^\s*(?:(?<int>\d+)|(?<name>\w+)|(?<illegal>\S+))\s*";

impl Reader {
    pub fn new() -> Self {
        match Regex::new(REGEX) {
            Err(err) => panic!("Failed regex: {:?}", err),
            Ok(regex) => Reader {regex},
        }
    }

    pub fn parse(&self, mut string: &str) -> Result<Vec<Frame>, Error> {
        let mut vec = Vec::<Frame>::new();
        while string.len() > 0 {
            let Some(captures) = self.regex.captures(string) else {
                return Err(Error::Illformed(String::from(string)))
            };
            let Some(m) = captures.get(0) else {
                panic!("Bad capture: {:?}", captures)
            };
            string = &string[m.end()..];
            vec.push(Self::convert(captures)?);
        };
        
        Ok(vec)
    }

    #[allow(dead_code)]
    pub fn result(&self, string: &str) -> Option<Vec<Frame>> {
        match self.parse(string) {
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
