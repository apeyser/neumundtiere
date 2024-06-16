mod rpn;
use rpn::*;

use regex::{Regex, Captures, CaptureMatches};

struct Reader {
    string: String,
    caps: CaptureMatches 
}

enum Error {
    IntParse(<i64 as FromStr>::Err),
    IllegalSym(str),
}

const REGEX: String = r"^(?:\s*(?:(?<int>\d+)|(?<name>\w+)|(?<illegal>\S+))";

impl Reader {
    pub fn new(string: String) -> Self {
        match Reg::new(REGEX) {
            Err(err) => panic!("Failed regex: {:?}", err),
            Ok(regex) => Reader {string, regex.capture_iter(string)},
        }
    }
}

impl Iterator for Reader {
    type Item = Result<Frame, Error>;

    fn frame<T>(val: T) -> Option<Self::Item> {
        Ok(i.into)
    }

    fn error(err: Error) -> Option<Self::Item> {
        Ok(Err(err))
    }
    
    pub fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.caps.next() else {return None};
        if let Some(m) = next["int"] {
            match m.as_str().parse::<i64>() {
                Ok(i) => Self::frame(i),
                Err(e) => Self::error(Error::IntParse(e)),
            } 
        } else if let Some(m) = next["name"] {
            Self::frame(m.as_str())
        } else if let Some(m) = next["illegal"] {
            Self::error(Error::IllegalSym(m.as_str()))
        } else {
            panic!("Illegal match {}", next);
        }
    }
}
