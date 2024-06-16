use std::fmt;
use std::convert::From;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Frame {
    Int(i64),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
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
            Frame::UnaryOp(op)  => writer(f, "UnaryOp", op),
            Frame::BinaryOp(op) => writer(f, "BinaryOp", op),
            Frame::Name(name)   => writer(f, "Name", name),
        }
    }
}

type UnaryOpFunc = fn(i64) -> i64;
#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    name: &'static str,
    op: UnaryOpFunc,
}

impl UnaryOp {
    fn exec(&self, f: Frame) -> Result<Frame, Error> {
        if let Frame::Int(i) = f {
            Ok(Frame::from((self.op)(i)))
        } else {
            Err(Error::OpType)
        }
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

type BinaryOpFunc = fn(i64, i64) -> i64;
#[derive(Debug, Copy, Clone)]
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
    fn exec(&self, f1: Frame, f2: Frame) -> Result<Frame, Error> {
        if let (Frame::Int(i1), Frame::Int(i2)) = (f1, f2) {
            Ok(Frame::from((self.op)(i1, i2)))
        } else {
            Err(Error::OpType)
        }
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
                ("neg".into(), NEG.into()),
                ("add".into(), ADD.into()),
                ("sub".into(), SUB.into()),
            ])
        }
    }

    pub fn get(&mut self, string: String) -> Option<Frame> {
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
    StackUnderflow,
    OpType,
    Unknown(String),
}

impl Vm {
    pub fn new() -> Self {
        let op_stack = Vec::new();
        let exec_stack = Vec::new();
        let dict = Dict::new();
        Vm { op_stack, exec_stack, dict }
    }

    pub fn exec<I>(&mut self, frames: I) -> Result<Option<&Frame>, Error>
        where I: Iterator<Item=Frame>
    {
        let mut frames: Vec<Frame> = frames.collect();
        frames.reverse();
        self.exec_stack.append(&mut frames);
        loop {
            let Some(frame) = self.exec_stack.pop() else {     
                break Ok(self.peek());
            };
            
            match frame {
                Frame::Name(s) => {
                    let Some(f) = self.dict.get(s.clone()) else {
                        return Err(Error::Unknown(s))
                    };
                    self.exec_stack.push(f);
                },
                
                Frame::Int(i) => {
                    let f = Frame::from(i);
                    self.op_stack.push(f);
                },

                Frame::UnaryOp(op) => {
                    let Some(f) = self.op_stack.pop() else {
                        break Err(Error::StackUnderflow);
                    };
                    let f = op.exec(f)?;
                    self.op_stack.push(f);
                },
                
                Frame::BinaryOp(op) => {
                    let len = self.op_stack.len();
                    if len < 2 {
                        break Err(Error::StackUnderflow)
                    };
                    
                    let [ref f1, ref f2] = self.op_stack[len-2..] else {
                        panic!("Impossible");
                    };
                    let f = op.exec(f1.clone(), f2.clone())?;
                    self.op_stack[len-2] = f;
                    self.op_stack.truncate(len-1);
                }
            }
        }
    }

    pub fn peek(&self) -> Option<&Frame> {
        self.op_stack.last()
    }

    #[allow(dead_code)]
    pub fn stack(&self) -> Vec<Frame> {
        self.op_stack.clone()
    }
}

    
