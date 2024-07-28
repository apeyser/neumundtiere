use std::str::FromStr;
use std::convert::From;
use regex::{Regex, RegexBuilder, Captures, Match};

use crate::vm::{self, *};
use crate::error::*;
use crate::numeric::{Number, NumericValue};
use crate::types::num::Num;
use crate::numeric::primitive::NumericPrimitive;

pub struct Reader<'a> {
    regex: Regex,
    array_f: Regex,
    array_u: Regex,
    array_i: Regex,
    vm: &'a mut Vm,
}

struct RegexStrings {
    array_f: String,
    array_u: String,
    array_i: String,
    regex: String,
}

impl RegexStrings {
    pub fn new() -> Self {
        let re_f = r"[+\-]?(?:(?:\d+[eE][+\-]?\d+)|(?:(?:\d+[.]\d*)|(?:[.]\d+))(?:[eE][+\-]?\d+)?)";
        let re_u = r"\d+";
        let re_i = r"[+\-]?\d+";

        let array_f = format!(r"\s+(?<value>{re_f})d?");
        let array_u = format!(r"\s+(?<value>{re_u})u?");
        let array_i = format!(r"\s+(?<value>{re_i})l?");
        
        let regex = format!(r"
^\s*
(?:
  (?:(?<float>{re_f})d?)
 |(?:(?<usize>{re_u})u)
 |(?:(?<int>{re_i})l?)
 |(?:<d(?:<farray>[^>]*)\s*>)
 |(?:<l(?:<iarray>[^>]*)\s*>)
 |(?:<u(?:<uarray>[^>]*)\s*>)
 |/(?<pname>[^\s/{{}}\[\]()]+)
 |(?<aname>[^\s/{{}}\[\]()]+)
 |\((?<string>(?:\\\(|[^\)])*)\)
 |(?<pmark>\[)
 |(?<mklist>\])
 |(?<amark>\{{)
 |(?<mkproc>\}})
 |(?<illegal>\S+)
)\s*
(?:[|][^\n]*(?:\n|$))?
");
        Self {
            array_f,
            array_u,
            array_i,
            regex,
        }
    }
}

fn mkregex(s: &String) -> Regex {
    match RegexBuilder::new(s.as_str()).ignore_whitespace(true).build() {
        Err(err) => panic!("Failed regex: {:?}, '{s}'", err),
        Ok(regex) => regex,
    }
}

trait ParseError: FromStr {
    fn error(err: Self::Err, s: &str) -> Error;
}

macro_rules! make_parse_error {
    ($($prim:ty => $ident:path),+) => {
        $(
            impl ParseError for $prim {
                fn error(err: Self::Err, s: &str) -> Error {
                    $ident(err, s.to_string())
                }
            }
        )+
    }
}

make_parse_error!(f64 => Error::FloatParse,
                  i64 => Error::IntParse,
                  usize => Error::USizeParse);

fn parse<T: ParseError>(m: Match) -> Result<T, Error> {
    match m.as_str().parse::<T>() {
        Ok(v) => Ok(v),
        Err(e) => Err(<T as ParseError>::error(e, m.as_str())),
    }
}

fn next_number<'a>(string: &'a str, re: &Regex) ->
    Result<(&'a str, Match<'a>), Error>
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

fn mkarray<T>(m: Match, re: &Regex) ->
    Result<Frame, Error> where
    T: ParseError + NumericPrimitive,
    Num: From<Number<T>>
{
    let mut string = m.as_str();
    let mut r = Vec::<NumericValue<T>>::new();
    while string.len() > 0 {
        let (nstring, m) = next_number(string, re)?;
        string = nstring;
        if m.as_str() == "*" {
            r.push(NumericValue::<T>::NaN)
        } else {
            r.push(NumericValue::<T>::Value(parse::<T>(m)?));
        }
    };
    Ok(Frame::Num(Number::<T>::Array(r).into()))
}

fn mkscalar<T>(m: Match) ->
    Result<Frame, Error> where
    T: NumericPrimitive + ParseError,
    Num: From<Number<T>>
{
    let value = parse::<T>(m)?;
    let value = NumericValue::<T>::from_primitive(value);
    Ok(Frame::Num(Number::<T>::Scalar(value).into()))
}

impl<'a> Reader<'a> {
    pub fn new(vm: &'a mut Vm) -> Self {
        let strings = RegexStrings::new();
        Self {
            regex:   mkregex(&strings.regex),
            array_f: mkregex(&strings.array_f),
            array_u: mkregex(&strings.array_u),
            array_i: mkregex(&strings.array_i),
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
            mkscalar::<f64>(m)
        } else if let Some(m) = captures.name("int") {
            mkscalar::<i64>(m)
        } else if let Some(m) = captures.name("usize") {
            mkscalar::<usize>(m)
        } else if let Some(m) = captures.name("farray") {
            mkarray::<f64>(m, &self.array_f)
        } else if let Some(m) = captures.name("iarray") {
            mkarray::<i64>(m, &self.array_i)
        } else if let Some(m) = captures.name("uarray") {
            mkarray::<usize>(m, &self.array_u)
        } else if let Some(_) = captures.name("pmark") {
            Ok(Passive::Mark.into())
        } else if let Some(_) = captures.name("mklist") {
            Ok(vm::ops::MKLIST.into())
        } else if let Some(_) = captures.name("amark") {
            Ok(Active::Mark.into())
        } else if let Some(_) = captures.name("mkproc") {
            Ok(vm::ops::MKPROC.into())
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
