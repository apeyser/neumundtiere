use std::fmt;

use itertools::Itertools;

use super::*;
use crate::error::Error;

pub trait Op {
    fn exec(&self, vm: &mut Vm) -> Option<Error>;
    fn mkpair(&self, table: &mut InternTable) -> (Name, Frame) {
        let (s, f) = self.from();
        (table.intern(s.into()), f)
    }
    fn from(&self) -> (&'static str, Frame);
}

type VmOpFunc = fn(stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VmOp {
    name: &'static str,
    op: VmOpFunc,
    n: usize,
}

impl VmOp {
    pub const fn new(name: &'static str, op: VmOpFunc, n: usize) -> Self {
        Self {name, op, n}
    }
}

impl fmt::Display for VmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Op for VmOp {
    fn from(&self) -> (&'static str, Frame) {
        (self.name, Frame::from(*self))
    }
    
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let n = self.n;
        let stack = &mut vm.op_stack;
        let len = stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        }

        let len = len-n;
        let substack = stack.split_off(len);
        match (self.op)(substack, vm) {
            Ok(mut substack) => {
                vm.op_stack.append(&mut substack);
                None
            },
            Err(error) => return Some(error),
        }
    }
}

type UnaryOpFunc = fn(Num) -> Result<Num, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnaryOp {
    name: &'static str,
    op: UnaryOpFunc,
}

impl UnaryOp {
    pub const fn new(name: &'static str, op: UnaryOpFunc) -> Self {
        Self {name, op}
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Op for UnaryOp {
    fn from(&self) -> (&'static str, Frame) {
        (self.name, Frame::from(*self))
    }
    
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let i = match vm.op_stack.pop() {
            None => return Error::StackUnderflow.into(),
            Some(Frame::Num(num)) => num,
            Some(_) => return Error::OpType.into(),
        };
        match (self.op)(i) {
            Ok(r) => {
                vm.op_stack.push(r.into());
                None
            },
            Err(e) => e.into(),
        }
    }
}

type BinaryOpFunc = fn(Num, Num) -> Result<Num, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BinaryOp {
    name: &'static str,
    op: BinaryOpFunc,
}

impl BinaryOp {
    pub const fn new(name: &'static str, op: BinaryOpFunc) -> Self {
        Self {name, op}
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Op for BinaryOp {
    fn from(&self) -> (&'static str, Frame) {
        (self.name, Frame::from(*self))
    }
    
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let stack = &mut vm.op_stack;
        let len = stack.len();
        if len < 2 {
            return Error::StackUnderflow.into()
        };

        let substack = stack.split_off(len-2); 
        let (Frame::Num(i1), Frame::Num(i2))
            = substack.into_iter().collect_tuple().unwrap()
        else {
            return Error::OpType.into()
        };

        match (self.op)(i1, i2) {
            Ok(r) => {
                stack.push(r.into());
                None
            },
            Err(e) => e.into(),
        }
    }
}

type NaryOpFunc = fn(stack: Vec<Frame>) -> Result<Vec<Frame>, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NaryOp {
    name: &'static str,
    op: NaryOpFunc,
    n: usize,
}

impl NaryOp {
    pub const fn new(name: &'static str, op: NaryOpFunc, n: usize) -> Self {
        Self {name, op, n}
    }
}

impl fmt::Display for NaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Op for NaryOp {
    fn from(&self) -> (&'static str, Frame) {
        (self.name, Frame::from(*self))
    }
    
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let stack = &mut vm.op_stack;
        let n = self.n;
        let len = stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        }

        let len = len-n;
        let substack = stack.split_off(len);
        let mut frames = match (self.op)(substack) {
            Ok(frames) => frames,
            Err(e) => return e.into(),
        };
        stack.append(&mut frames);
        None
    }
}

type StackOpFunc = fn(&Vec<Frame>, Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StackOp {
    name: &'static str,
    op: StackOpFunc,
    n: usize,
}

impl StackOp {
    pub const fn new(name: &'static str, op: StackOpFunc, n: usize) -> Self {
        Self {name, op, n}
    }
}

impl fmt::Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Op for StackOp {
    fn from(&self) -> (&'static str, Frame) {
        (self.name, Frame::from(*self))
    }
    
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let stack = &mut vm.op_stack;
        let n = self.n;
        let len = stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        }

        let len = len-n;
        let substack = stack.split_off(len);
        let (mut substack, n) = match (self.op)(stack, substack) {
            Ok((substack, n)) => (substack, n),
            Err(e) => return e.into(),
        };
        if len < n {
            return Error::StackUnderflow.into()
        };
        stack.truncate(len-n);
        stack.append(&mut substack);
        None
    }
}
