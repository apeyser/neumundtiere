use std::fmt;
use std::convert::From;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Int(i64),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    StackOp(StackOp),
    Name(String),
}

impl From<i64> for Frame {
    fn from(item: i64) -> Self {
        Frame::Int(item)
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

fn writer<T: fmt::Display>(f: &mut fmt::Formatter<'_>,
                           s: &'static str,
                           v: &T) -> fmt::Result
{
    write!(f, "{}: {}", s, v)
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Frame::Int(v)       => writer(f, "Int", v),
            Frame::StackOp(op)  => writer(f, "StackOp", op),
            Frame::UnaryOp(op)  => writer(f, "UnaryOp", op),
            Frame::BinaryOp(op) => writer(f, "BinaryOp", op),
            Frame::Name(name)   => writer(f, "Name", name),
        }
    }
}

type UnaryOpFunc = fn(i64) -> i64;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnaryOp {
    name: &'static str,
    op: UnaryOpFunc,
}

impl UnaryOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let Some(f) = vm.op_stack.pop() else {
            return Error::StackUnderflow.into()
        };
        let Frame::Int(i) = f else {
            return Error::OpType.into()
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

fn fneg(v: i64) -> i64 { -v }
pub const NEG: UnaryOp = UnaryOp {
    name: "neg",
    op: fneg,
};

///

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
    println!("Stack: {stack:?}");
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


type BinaryOpFunc = fn(i64, i64) -> i64;
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

        let (&Frame::Int(i1), &Frame::Int(i2)) = (f1, f2) else {
            return Error::OpType.into()
        };
        let r = (self.op)(i1, i2);
        vm.op_stack[len-2] = r.into();
        vm.op_stack.truncate(len-1);
        None
    }
}

fn fadd(a: i64, b: i64) -> i64 { a + b }
pub const ADD: BinaryOp = BinaryOp {
    name: "add",
    op: fadd,
};

fn fsub(a: i64, b: i64) -> i64 { a - b }
pub const SUB: BinaryOp = BinaryOp {
    name: "sub",
    op: fsub,
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
                ("pop".into(),  POP.into()),
                ("dup".into(),  DUP.into()),
                ("exch".into(), EXCH.into()),
                ("==".into(),   SHOW.into()),
                ("=".into(),    PEEK.into()),
                ("quit".into(), QUIT.into()),
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

#[derive(Debug)]
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
                
                Frame::Int(i) => {
                    let f = Frame::from(i);
                    self.op_stack.push(f);
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
                }

                Frame::StackOp(op) => {
                    if let Some(e) = op.exec(self) {
                        return e.into()
                    }
                }
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
            3.into(),
            1.into(),
            ADD.into(),
            NEG.into(),
            2.into(),
            SUB.into(),
            NEG.into(),
        ];
        let None = vm.result(cmd) else {
            panic!("Result: error")
        };

        let reader = Reader::new();
        let Some(frames) = reader.result("1 2 add 3 sub") else {
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
            3.into(),
            1.into(),
            ADD.into(),
            NEG.into(),
            2.into(),
            SUB.into(),
            NEG.into(),
        ];
        match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Int(6), frame),
        };

        let reader = Reader::new();
        match reader.parse("1 2 add 4 sub") {
            Err(e)  => panic!("Parse error: {e:?}"),
            Ok(frames) => match vm.exec(frames) {
                Err(e) => panic!("Error {e:?}"),
                Ok(None) => panic!("Empty stack"),
                Ok(Some(frame)) => assert_eq!(Frame::Int(-1), frame),
            },
        };
    }

    #[test]
    fn stacked() {
        let mut vm = Vm::new();
        let frames: Vec<Frame> = vec![
            3.into(),
            DUP.into(),
            ADD.into(),
            NEG.into(),
            2.into(),
            SUB.into(),
            NEG.into(),
        ];
        match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Int(8), frame),
        };

        let reader = Reader::new();
        match reader.parse("1 dup add 4 pop dup sub") {
            Err(e)  => panic!("Parse error: {e:?}"),
            Ok(frames) => match vm.exec(frames) {
                Err(e) => panic!("Error {e:?}"),
                Ok(None) => panic!("Empty stack"),
                Ok(Some(frame)) => assert_eq!(Frame::Int(0), frame),
            },
        };
    }
}
