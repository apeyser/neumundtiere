use std::error;
use std::fmt::{self, Display, Formatter};
use std::convert::From;
use std::collections::HashMap;
use std::ops::{Add,Sub,Div,Mul,Neg};

fn writer<T: fmt::Display>(f: &mut fmt::Formatter<'_>,
                           s: &'static str,
                           v: &T) -> fmt::Result
  {write!(f, "{}: {}", s, v)}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Num {
    Int(i64),
    NaN,
}

impl From<i64> for Num {
    fn from(item: i64) -> Self {Num::Int(item)}
}

impl Neg for Num {
    type Output = Num;
    fn neg(self) -> Self::Output {
        match self {
            Num::NaN => Num::NaN,
            Num::Int(i) => (-i).into(),
        }
    }
}

impl Add for Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => (a+b).into(),
            (Num::NaN, _) => Num::NaN,
            (_, Num::NaN) => Num::NaN,
        }
    }
}

impl Sub for Num {
    type Output = Num;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => (a-b).into(),
            (Num::NaN, _) => Num::NaN,
            (_, Num::NaN) => Num::NaN,
        }
    }
}

impl Mul for Num {
    type Output = Num;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => (a*b).into(),
            (Num::NaN, _) => Num::NaN,
            (_, Num::NaN) => Num::NaN,
        }
    }
}

impl Div for Num {
    type Output = Num;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (_, Num::Int(0)) => Num::NaN,
            (Num::Int(a), Num::Int(b)) => (a/b).into(),
            (Num::NaN, _) => Num::NaN,
            (_, Num::NaN) => Num::NaN,
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Num::NaN => write!(f, "NaN"),
            Num::Int(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Num(Num),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    StackOp(StackOp),
    Name(String),
}

impl From<Num> for Frame {
    fn from(item: Num) -> Self {
        Frame::Num(item)
    }
}

impl From<UnaryOp> for Frame {
    fn from(item: UnaryOp) -> Self {
        Frame::UnaryOp(item)
    }
}

impl From<BinaryOp> for Frame {
    fn from(item: BinaryOp) -> Self {
        Frame::BinaryOp(item)
    }
}

impl From<StackOp> for Frame {
    fn from(item: StackOp) -> Self {
        Frame::StackOp(item)
    }
}

impl From<String> for Frame {
    fn from(item: String) -> Self {
        Frame::Name(item)
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Frame::Num(v)       => writer(f, "Num", v),
            Frame::StackOp(op)  => writer(f, "StackOp", op),
            Frame::UnaryOp(op)  => writer(f, "UnaryOp", op),
            Frame::BinaryOp(op) => writer(f, "BinaryOp", op),
            Frame::Name(name)   => writer(f, "Name", name),
        }
    }
}

type UnaryOpFunc = fn(Num) -> Num;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnaryOp {
    name: &'static str,
    op: UnaryOpFunc,
}

impl UnaryOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let i = match vm.op_stack.pop() {
            None => return Error::StackUnderflow.into(),
            Some(Frame::Num(num)) => num,
            Some(_) => return Error::OpType.into(),
        };
        let r = (self.op)(i);
        vm.op_stack.push(r.into());
        None
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub const NEG: UnaryOp = UnaryOp {
    name: "neg",
    op: |a| -a,
};

type StackOpFunc = fn(&Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StackOp {
    name: &'static str,
    op: StackOpFunc,
}

impl StackOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let stack = &mut vm.op_stack;
        let (mut substack, n) = match (self.op)(stack) {
            Ok((substack, n)) => (substack, n),
            Err(e) => return e.into(),
        };
        let len = stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        };
        stack.truncate(len-n);
        stack.append(&mut substack);
        None
    }
}

impl fmt::Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn fpop(_stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Ok((vec![], 1))
}
pub const POP: StackOp = StackOp {
    name: "pop",
    op: fpop,
};

fn fclear(stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Ok((vec![], stack.len()))
}
pub const CLEAR: StackOp = StackOp {
    name: "clear",
    op: fclear,
};

fn fdup(stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    let len = stack.len();
    if len == 0 {
        return Error::StackUnderflow.into()
    };
    Ok((stack[len-1..].into(), 0))
}
pub const DUP: StackOp = StackOp {
    name: "dup",
    op: fdup,
};

fn fexch(stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    let len = stack.len();
    if len < 2 {
        return Error::StackUnderflow.into()
    };
    let [ref f1, ref f2] = stack[len-2..] else {panic!("Impossible")};
    Ok((vec![f2.clone(), f1.clone()], 2))
}
pub const EXCH: StackOp = StackOp {
    name: "exch",
    op: fexch,
};

fn fshow(stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    println!("Stack:");
    for v in stack.into_iter().rev() {
        println!("  {v}");
    };
    Ok((vec![], 0))
}
pub const SHOW: StackOp = StackOp {
    name: "==",
    op: fshow,
};

fn fpeek(stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    match stack.last() {
        None => println!("Stack: empty"),
        Some(frame) => println!("Top: {frame}"),
    };
    Ok((vec![], 0))
}
pub const PEEK: StackOp = StackOp {
    name: "=",
    op: fpeek,
};

fn fquit(_stack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Err(Error::Quit)
}
pub const QUIT: StackOp = StackOp {
    name: "quit",
    op: fquit,
};

type BinaryOpFunc = fn(Num, Num) -> Num;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BinaryOp {
    name: &'static str,
    op: BinaryOpFunc,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl BinaryOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let len = vm.op_stack.len();
        if len < 2 {
            return Error::StackUnderflow.into()
        };
                    
        let [ref f1, ref f2] = vm.op_stack[len-2..] else {
            panic!("Impossible");
        };

        let (&Frame::Num(i1), &Frame::Num(i2)) = (f1, f2) else {
            return Error::OpType.into()
        };
        let r = (self.op)(i1, i2);
        vm.op_stack[len-2] = r.into();
        vm.op_stack.truncate(len-1);
        None
    }
}

pub const ADD: BinaryOp = BinaryOp {
    name: "add",
    op: |a, b| a+b,
};

pub const SUB: BinaryOp = BinaryOp {
    name: "sub",
    op: |a, b| a-b,
};

pub const MUL: BinaryOp = BinaryOp {
    name: "mul",
    op: |a, b| a*b,
};

pub const DIV: BinaryOp = BinaryOp {
    name: "div",
    op: |a, b| a/b,
};

type DictBase = HashMap<String, Frame>;

struct Dict {
    dict: DictBase
}

impl Dict {
    pub fn new() -> Self {
        Dict {
            dict: DictBase::from([
                ("neg".into(),  NEG.into()),
                ("add".into(),  ADD.into()),
                ("+".into(),    ADD.into()),
                ("sub".into(),  SUB.into()),
                ("-".into(),    SUB.into()),
                ("mul".into(),  MUL.into()),
                ("*".into(),    MUL.into()),
                ("div".into(),  DIV.into()),
                ("/".into(),    DIV.into()),
                ("NaN".into(),  Num::NaN.into()),
                ("pop".into(),  POP.into()),
                ("dup".into(),  DUP.into()),
                ("exch".into(), EXCH.into()),
                ("==".into(),   SHOW.into()),
                ("=".into(),    PEEK.into()),
                ("quit".into(), QUIT.into()),
                ("clear".into(), CLEAR.into()),
            ])
        }
    }

    pub fn get(&self, string: String) -> Option<Frame> {
        self.dict.get::<String>(&string).cloned()
    }
}

pub struct Vm {
    op_stack: Vec<Frame>,
    exec_stack: Vec<Frame>,
    dict: Dict,
}

#[derive(Debug, Clone)]
pub enum Error {
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
        }
    }
}

impl error::Error for Error {}
 
impl Vm {
    pub fn new() -> Self {
        Vm {
            op_stack: Vec::new(),
            exec_stack: Vec::new(),
            dict: Dict::new(),
        }
    }

    pub fn exec(&mut self, mut frames: Vec<Frame>) -> Result<Option<Frame>, Error>
    {
        frames.reverse();
        self.exec_stack.append(&mut frames);
        loop {
            let Some(frame) = self.exec_stack.pop() else {     
                break Ok(self.peek());
            };
            
            match frame {
                Frame::Name(s) => {
                    let Some(f) = self.dict.get(s.clone()) else {
                        return Error::Unknown(s).into()
                    };
                    self.exec_stack.push(f);
                },

                Frame::UnaryOp(op) => {
                    if let Some(e) = op.exec(self) {
                        return e.into()
                    }
                },
                
                Frame::BinaryOp(op) => {
                    if let Some(e) = op.exec(self) {
                        return e.into()
                    }
                },

                Frame::StackOp(op) => {
                    if let Some(e) = op.exec(self) {
                        return e.into()
                    }
                },

                other => self.op_stack.push(other),
            }
        }
    }

    pub fn peek(&self) -> Option<Frame> {
        let frame = self.op_stack.last()?;
        Some(frame.clone())
    }

    #[allow(dead_code)]
    pub fn stack(&self) -> &Vec<Frame> {
        &self.op_stack
    }

    #[allow(dead_code)]
    pub fn result(&mut self, frames: Vec<Frame>) -> Option<Error> {
        match self.exec(frames) {
            Ok(Some(f)) => {
                println!("Result: {f}");
                None
            },
            Ok(None) => {
                println!("Empty stack");
                None
            },
            Err(e) => {
                println!("Error: {e:?}");
                Some(e)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Reader ;

    #[test]
    fn result() {
        let mut vm = Vm::new();

        let cmd: Vec<Frame> = vec![
            Num::Int(3).into(),
            Num::Int(1).into(),
            ADD.into(),
            NEG.into(),
            Num::Int(2).into(),
            SUB.into(),
            NEG.into(),
        ];
        let None = vm.result(cmd) else {
            panic!("Result: error")
        };

        let reader = Reader::new();
        let Some(frames) = reader.result(String::from("1 2 add 3 sub")) else {
            panic!("Result: read error")
        };
        let None = vm.result(frames) else {
            panic!("Result: error")
        };
    }

    #[test]
    fn exec() {
        let mut vm = Vm::new();
        let frames: Vec<Frame> = vec![
            Num::Int(3).into(),
            Num::Int(1).into(),
            ADD.into(),
            NEG.into(),
            Num::Int(2).into(),
            SUB.into(),
            NEG.into(),
        ];
        match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Num(6.into()), frame),
        };

        let reader = Reader::new();
        match reader.parse(String::from("1 2 add 4 sub")) {
            Err(e)  => panic!("Parse error: {e:?}"),
            Ok(frames) => match vm.exec(frames) {
                Err(e) => panic!("Error {e:?}"),
                Ok(None) => panic!("Empty stack"),
                Ok(Some(frame)) => assert_eq!(Frame::Num((-1).into()), frame),
            },
        };
    }

    #[test]
    fn stacked() {
        let mut vm = Vm::new();
        let frames: Vec<Frame> = vec![
            Num::Int(3).into(),
            DUP.into(),
            ADD.into(),
            NEG.into(),
            Num::Int(2).into(),
            SUB.into(),
            NEG.into(),
        ];
        match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Num(8.into()), frame),
        };

        let reader = Reader::new();
        match reader.parse(String::from("1 dup add 4 pop dup sub")) {
            Err(e)  => panic!("Parse error: {e:?}"),
            Ok(frames) => match vm.exec(frames) {
                Err(e) => panic!("Error {e:?}"),
                Ok(None) => panic!("Empty stack"),
                Ok(Some(frame)) => assert_eq!(Frame::Num(0.into()), frame),
            },
        };
    }
}
