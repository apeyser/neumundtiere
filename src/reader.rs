use regex::{Regex, RegexBuilder, Captures, Match};

use super::vm::*;
use super::error::*;
use super::vmops;

pub struct Reader<'a> {
    regex: Regex,
    array_f: Regex,
    array_u: Regex,
    array_i: Regex,
    vm: &'a mut Vm,
}

static FLOAT: &str = r"[+\-]?(?:(?:\d+[eE][+\-]?\d+)|(?:(?:\d+[.]\d*)|(?:[.]\d+))(?:[eE][+\-]?\d+)?)"
static USIZE: &str = r"\d+";
static INT: &str = r"[+\-]?\d+";

static ARRAY_F_REGEX: &str = format!(r"\s+(?<value>{FLOAT})d?").as_str();
static ARRAY_U_REGEX: &str = format!(r"\s+(?<value>{IUSIZE})u?").as_str();
static ARRAY_I_REGEX: &str = format!(r"\s+(?<value>{INT})l?").as_str();

static REGEX: &str = format!(r"
^\s*
(?:
  (?:(?<float>{FLOAT})d?)
 |(?:(?<usize>{USIZE})u)
 |(?:(?<int>{INT})l?)
 |(?<farray><d(?:<array>[^>]*)\s*>)
 |(?<iarray><l(?:<array>[^>]*)\s*>)
 |(?<uarray><u(?:<array>[^>]*)\s*>)
 |/(?<pname>[^\s/{}\[\]()]+)
 |(?<aname>[^\s/{}\[\]()]+)
 |\((?<string>(?:\\\(|[^\)])*)\)
 |(?<pmark>\[)
 |(?<mklist>\])
 |(?<amark>\{)
 |(?<mkproc>\})
 |(?<illegal>\S+)
)\s*
(?:[|][^\n]*(?:\n|$))?
").as_str();

fn mkregex(s: &str) -> Regex {
    match RegexBuilder::new(s).ignore_whitespace(true).build() {
        Err(err) => panic!("Failed regex: {:?}, '{s}'", err),
        Ok(regex) => regex,
    }
}

trait ParseError {
    fn error(err: FromStr::Err, s: &str) -> Error;
}

trait ParseError for f64 {
    fn error(err: FromStr::Err, s: String) -> Error {
        Error::FloatParse(err, s)
    }
}

trait ParseError for i64 {
    fn error(err: FromStr::Err, s: String) -> Error {
        Error::IntParse(err, s)
    }
}

trait ParseError for i64 {
    fn error(err: FromStr::Err, s: String) -> Error {
        Error::USizeParse(err, s)
    }
}

fn parse<T: ParseError>(m: Match) -> Result<T, Error> {
    match m.as_str().parse::<T>() {
        Ok(v) => Ok(v),
        Err(e) => Err(<T as ParseError>::error(e, String::from(m.as_str()))),
    }
}

fn next_number<'a>(string: &'a str, re: &Regex) ->
    Result<(&'a str, Match), Error>
{
    let Some(captures) = re.captures(string) else {
        return Err(Error::Illformed(String::from(string)))
    };
    let Some(m) = captures.get(0) else {
        panic!("Bad capture: {:?}", captures)
    };
    let string = &string[m.end()..];
    let Some(m) = captures.name("value") else {
        panic!("Bad value capture: {:?}", captures)
    };
    Ok((string, m)) 
}

fn mkarray<T: ParseError>(string: &str, re: &Regex) ->
    Result<Vec<T>, Error>
{
    let mut string = string;
    let r = Vec::<T>::new();
    while string.len() > 0 {
        let (nstring, m) = next_number(string, re)?;
        string = nstring;
        r.push(parse::<T>(m)?);
    };
    r
}

impl<'a> Reader<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        Self {
            regex: mkregex(REGEX),
            array_f: mkregex(ARRAY_F_REGEX),
            array_u: mkregex(ARRAY_U_REGEX),
            array_i: mkregex(ARRAY_I_REGEX),
            vm,
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
        if let Some(m) = captures.name("float") {
            Ok(Frame::Num(parse::<f64>(m)?.into()))
        } else if let Some(m) = captures.name("int") {
            Ok(Frame::Num(parse::<i64>(m)?.into()))
        } else if let Some(m) = captures.name("usize") {
            Ok(Frame::Num(parse::<usize>(m)?.into()))
        } else if let Some(m) = captures.name("farray") {
            Ok(Frame::Num(mkarray::<f64>(m.as_str(), &self.array_f)?.into()))
        } else if let Some(m) = captures.name("iarray") {
            Ok(Frame::Num(mkarray::<i64>(m.as_str(), &self.array_i)?.into()))
        } else if let Some(m) = captures.name("uarray") {
            Ok(Frame::Num(array::<usize>(m.as_str(), &self.array_u)?.into()))
        } else if let Some(_) = captures.name("pmark") {
            Ok(Passive::Mark.into())
        } else if let Some(_) = captures.name("mklist") {
            Ok(vmops::MKLIST.into())
        } else if let Some(_) = captures.name("amark") {
            Ok(Active::Mark.into())
        } else if let Some(_) = captures.name("mkproc") {
            Ok(vmops::MKPROC.into())
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
