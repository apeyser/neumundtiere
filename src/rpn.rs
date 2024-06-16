use std::fmt;
use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub enum Frame {
    Int(i64),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
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

#[allow(dead_code)]
fn fneg(v: i64) -> i64 { -v }
#[allow(dead_code)]
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

#[allow(dead_code)]
fn fadd(a: i64, b: i64) -> i64 { a + b }
#[allow(dead_code)]
pub const ADD: BinaryOp = BinaryOp {
    name: "add",
    op: fadd,
};

#[allow(dead_code)]
fn fsub(a: i64, b: i64) -> i64 { a - b }
#[allow(dead_code)]
pub const SUB: BinaryOp = BinaryOp {
    name: "sub",
    op: fsub,
};

pub struct Vm {
    stack: Vec<Frame>,
}

#[derive(Debug)]
pub enum Error {
    StackUnderflow,
    OpType,
}

impl Vm {
    pub fn new() -> Self {
        let stack = Vec::new();
        Vm { stack }
    }

    pub fn exec(&mut self, frames: Vec<Frame>) -> Result<Option<&Frame>, Error> {
        for frame in frames {
            match frame {
                Frame::Int(i) => {
                    let f = Frame::from(i);
                    self.stack.push(f);
                },

                Frame::UnaryOp(op) => {
                    let Some(f) = self.stack.pop() else {
                        return Err(Error::StackUnderflow);
                    };
                    let f = op.exec(f)?;
                    self.stack.push(f);
                },
                
                Frame::BinaryOp(op) => {
                    let len = self.stack.len();
                    if len < 2 {
                        return Err(Error::StackUnderflow)
                    };
                    
                    let [ref f1, ref f2] = self.stack[len-2..] else {
                        panic!("Impossible");
                    };
                    let f = op.exec(*f1, *f2)?;
                    self.stack[len-2] = f;
                    self.stack.truncate(len-1);
                }
            }
        }
        Ok(self.peek())
    }

    pub fn peek(&self) -> Option<&Frame> {
        self.stack.last()
    }

    #[allow(dead_code)]
    pub fn stack(&self) -> Vec<Frame> {
        self.stack.clone()
    }
}

    
