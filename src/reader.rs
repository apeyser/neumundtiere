use regex::{Regex, RegexBuilder, Captures};

use super::rpn::*;
use super::error::*;

pub struct Reader<'a> {
    regex: Regex,
    vm: &'a mut Vm,
}

static REGEX: &str = r"
^\s*
(?:
  (?<NaN>[*])
 |(?<float>[+\-]?(?:(?:\d+[eE][+\-]?\d+)|(?:(?:\d+[.]\d*)|(?:[.]\d+))(?:[eE][+\-]?\d+)?))
 |(?<int>[+\-]?\d+)
 |/(?<pname>[^\s/{}\[\]()]+)
 |(?<aname>[^\s/{}\[\]()]+)
 |\((?<string>(?:\\\(|[^\)])*)\)
 |(?<mark>\[)
 |(?<mklist>\])
 |(?<illegal>\S+)
)\s*
(?:[|][^\n]*(?:\n|$))?
";


impl<'a> Reader<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        match RegexBuilder::new(REGEX).ignore_whitespace(true).build() {
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

    pub fn exec(&mut self, frames: Vec<Frame>) -> Result<Option<Frame>, Error> {
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
        if let Some(_) = captures.name("NaN") {
            Ok(Num::NaN.into())
        } else if let Some(m) = captures.name("float") {
            match m.as_str().parse::<f64>() {
                Ok(f)  => Ok(Frame::Num(f.into())),
                Err(e) => Err(Error::FloatParse(e, String::from(m.as_str()))),
            }
        } else if let Some(m) = captures.name("int") {
            match m.as_str().parse::<i64>() {
                Ok(i) => Ok(Frame::Num(i.into())),
                Err(e) => Err(Error::IntParse(e, String::from(m.as_str()))),
            }
        } else if let Some(_) = captures.name("mark") {
            Ok(MARK.into())
        } else if let Some(_) = captures.name("mklist") {
            Ok(MKLIST.into())
        } else if let Some(m) = captures.name("pname") {
            Ok(Passive::Name(self.vm.intern(String::from(m.as_str()))).into())
        } else if let Some(m) = captures.name("aname") {
            Ok(Active::Name(self.vm.intern(String::from(m.as_str()))).into())
        } else if let Some(m) = captures.name("string") {
            Ok(Passive::String(String::from(m.as_str())).into())
        } else if let Some(m) = captures.name("illegal") {
            Err(Error::IllegalSym(m.as_str().into()))
        } else {
            panic!("Illegal match {:?}", captures);
        }
    }
}
