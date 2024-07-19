use std::fmt;
use std::convert::From;
use std::collections::HashMap;

use super::term;
use super::error::*;
use super::reader::Reader;
use super::name::{Name, InternTable};

pub use super::num::Num;

#[derive(Debug, Clone, PartialEq)]
pub enum Active {
    String(String),
    Name(Name),
    Mark,
    List(Vec<Frame>),
}

impl From<String> for Active {
    fn from(item: String) -> Self {
        Active::String(item)
    }
}

impl From<Name> for Active {
    fn from(item: Name) -> Self {
        Active::Name(item)
    }
}

impl From<Vec<Frame>> for Active {
    fn from(item: Vec<Frame>) -> Self {
        Active::List(item)
    }
}

impl fmt::Display for Active {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Active::String(string) => write!(f, "~({string})"),
            Active::Name(name) => write!(f, "~/({name})"),
            Active::Mark => write!(f, "{{"),
            Active::List(frames) => {
                write!(f, "{{ ")?;
                for frame in frames {
                    write!(f, "{frame} ")?
                };
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Passive {
    String(String),
    Name(Name),
    Mark,
    List(Vec<Frame>),
}

impl From<String> for Passive {
    fn from(item: String) -> Self {
        Passive::String(item)
    }
}

impl From<Name> for Passive {
    fn from(item: Name) -> Self {
        Passive::Name(item)
    }
}

impl From<Vec<Frame>> for Passive {
    fn from(item: Vec<Frame>) -> Self {
        Passive::List(item)
    }
}

impl fmt::Display for Passive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Passive::String(string) => write!(f, "({string})"),
            Passive::Name(name) => write!(f, "/({name})"),
            Passive::Mark => write!(f, "["),
            Passive::List(frames) => {
                write!(f, "[ ")?;
                for frame in frames {
                    write!(f, "{frame} ")?
                };
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Num(Num),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    StackOp(StackOp),
    VmOp(VmOp),
    Active(Active),
    Passive(Passive),
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

impl From<VmOp> for Frame {
    fn from(item: VmOp) -> Self {
        Frame::VmOp(item)
    }
}

impl From<Passive> for Frame {
    fn from(item: Passive) -> Self {
        Frame::Passive(item)
    }
}

impl From<Active> for Frame {
    fn from(item: Active) -> Self {
        Frame::Active(item)
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Frame::Num(v)       => write!(f, "{v}"),
            Frame::StackOp(op)  => write!(f, "{op}"),
            Frame::VmOp(op)     => write!(f, "{op}"),
            Frame::UnaryOp(op)  => write!(f, "{op}"),
            Frame::BinaryOp(op) => write!(f, "{op}"),
            Frame::Active(frame)  => write!(f, "{frame}"),
            Frame::Passive(frame) => write!(f, "{frame}"),
        }
    }
}

trait Op {
    fn exec(&self, vm: &mut Vm) -> Option<Error>;
}

type UnaryOpFunc = fn(Num) -> Num;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnaryOp {
    name: &'static str,
    op: UnaryOpFunc,
}

impl Op for UnaryOp {
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

type StackOpFunc = fn(&Vec<Frame>, &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StackOp {
    name: &'static str,
    op: StackOpFunc,
    n: usize,
}

impl Op for StackOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let stack = &mut vm.op_stack;
        let n = self.n;
        let len = stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        }

        let len = len-n;
        let substack = stack.split_off(len);
        let (mut substack, n) = match (self.op)(stack, &substack) {
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

impl fmt::Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn fpop(_stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    Ok((vec![], 0))
}
pub const POP: StackOp = StackOp {
    name: "pop",
    op: fpop,
    n: 1,
};

fn fclear(stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Ok((vec![], stack.len()))
}
pub const CLEAR: StackOp = StackOp {
    name: "clear",
    op: fclear,
    n: 0,
};

fn fdup(_stack: &Vec<Frame>, substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    let last = &substack[0];
    Ok((vec![last.clone(), last.clone()], 0))
}
pub const DUP: StackOp = StackOp {
    name: "dup",
    op: fdup,
    n: 1,
};

fn fexch(_stack: &Vec<Frame>, substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    let [ref f1, ref f2] = substack[..2] else {panic!("Impossible")};
    Ok((vec![f2.clone(), f1.clone()], 0))
}
pub const EXCH: StackOp = StackOp {
    name: "exch",
    op: fexch,
    n: 2,
};

fn fshow(stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    println!("Stack:");
    for v in stack.into_iter().rev() {
        println!("  {v}");
    };
    println!("-----");
    Ok((vec![], 0))
}
pub const SHOW: StackOp = StackOp {
    name: "==",
    op: fshow,
    n: 0,
};

fn fpeek(stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    match stack.last() {
        None => println!("Stack: empty"),
        Some(frame) => println!("Top: {frame}"),
    };
    Ok((vec![], 0))
}
pub const PEEK: StackOp = StackOp {
    name: "=",
    op: fpeek,
    n: 0,
};

fn fmark(_stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    Ok((vec![Passive::Mark.into()], 0))
}
pub const MARK: StackOp = StackOp {
    name: "mark",
    op: fmark,
    n: 0,
};

fn fmklist(stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    let mark = Frame::Passive(Passive::Mark);
    let r: Vec<Frame>
        = stack.into_iter().cloned().rev()
               .take_while(|frame| *frame != mark)
               .collect::<Vec<Frame>>().into_iter()
               .rev().collect();
    
    if r.len() == stack.len() {
        Err(Error::StackUnderflow)
    } else {
        Ok((vec![Frame::Passive(r.clone().into())], r.len()+1))
    }
}
pub const MKLIST: StackOp = StackOp {
    name: "mklist",
    op: fmklist,
    n: 0,
};

fn fquit(_stack: &Vec<Frame>, _substack: &Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    Err(Error::Quit)
}
pub const QUIT: StackOp = StackOp {
    name: "quit",
    op: fquit,
    n: 0,
};

type VmOpFunc = fn(_stack: &Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VmOp {
    name: &'static str,
    op: VmOpFunc,
    n: usize,
}

impl Op for VmOp {
    fn exec(&self, vm: &mut Vm) -> Option<Error> {
        let n = self.n;
        let len = vm.op_stack.len();
        if len < n {
            return Error::StackUnderflow.into()
        }

        let len = len-n;
        let substack = vm.op_stack.split_off(len);
        match (self.op)(&substack, vm) {
            Ok(mut substack) => {
                vm.op_stack.append(&mut substack);
                None
            },
            Err(error) => return Some(error),
        }
    }
}

impl fmt::Display for VmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn name_op(stack: &Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    match (stack[0].clone(), stack[1].clone()) {
        (frame, Frame::Passive(Passive::Name(name))) => {
            vm.dict.insert(name, frame);
            Ok(vec![])
        },
        _ => Err(Error::OpType),
    }           
}
pub const NAME: VmOp = VmOp {
    name: "name",
    op: |stack, vm| name_op(stack, vm),
    n: 2,
};

fn mkname(stack: &Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    match stack[0].clone() {
        Frame::Passive(Passive::String(string))
            => Ok(vec![Passive::Name(vm.intern_table.intern(string)).into()]),
        _ => Err(Error::OpType),
    }       
}
pub const MKNAME: VmOp = VmOp {
    name: "mkname",
    op: |stack, vm| mkname(stack, vm),
    n: 1,
};

fn mkstr(stack: &Vec<Frame>, _vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    match stack[0].clone() {
        Frame::Passive(Passive::Name(name))
            => Ok(vec![Frame::Passive(name.into())]),
        _ => Err(Error::OpType),
    }
}
pub const MKSTR: VmOp = VmOp {
    name: "mkname",
    op: |stack, vm| mkstr(stack, vm),
    n: 1,
};

fn mkpass(stack: &Vec<Frame>, _vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let passive = match stack[0].clone() {
        Frame::Active(active) => match active {
            Active::Name(name) => Passive::Name(name),
            Active::String(string) => Passive::String(string),
            Active::Mark => Passive::Mark,
            Active::List(list) => Passive::List(list),
        }
        _ => return Err(Error::OpType),
    };
    Ok(vec![Frame::Passive(passive)])
}
pub const MKPASS: VmOp = VmOp {
    name: "mkpass",
    op: |stack, vm| mkpass(stack, vm),
    n: 1,
};

fn mkact(stack: &Vec<Frame>, _vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let active = match stack[0].clone() {
        Frame::Passive(passive) => match passive {
            Passive::Name(name) => Active::Name(name),
            Passive::String(string) => Active::String(string),
            Passive::Mark => Active::Mark,
            Passive::List(list) => Active::List(list),
        }
        _ => return Err(Error::OpType),
    };
    Ok(vec![Frame::Active(active)])
}
pub const MKACT: VmOp = VmOp {
    name: "mkact",
    op: |stack, vm| mkact(stack, vm),
    n: 1,
};

fn op_exec(stack: &Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    match stack[0].clone() {
        frame@Frame::Active(_) => {
            vm.exec_stack.push(frame);
            Ok(vec![])
        },
        _ => Err(Error::OpType),
    }
}
pub const EXEC: VmOp = VmOp {
    name: "exec",
    op: |stack, vm| op_exec(stack, vm),
    n: 1,
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

impl Op for BinaryOp {
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

type DictBase = HashMap<Name, Frame>;

struct Dict {
    dict: DictBase,
}

impl Dict {
    pub fn new(t: &mut InternTable) -> Self {
        Dict {
            dict: HashMap::from([
                (t.intern("neg".into()),    NEG.into()),
                (t.intern("add".into()),    ADD.into()),
                (t.intern("+".into()),      ADD.into()),
                (t.intern("sub".into()),    SUB.into()),
                (t.intern("-".into()),      SUB.into()),
                (t.intern("mul".into()),    MUL.into()),
                (t.intern("×".into()),      MUL.into()),
                (t.intern("div".into()),    DIV.into()),
                (t.intern("÷".into()),      DIV.into()),
                (t.intern("NaN".into()),    Num::NaN.into()),
                (t.intern("pop".into()),    POP.into()),
                (t.intern("dup".into()),    DUP.into()),
                (t.intern("exch".into()),   EXCH.into()),
                (t.intern("==".into()),     SHOW.into()),
                (t.intern("=".into()),      PEEK.into()),
                (t.intern("quit".into()),   QUIT.into()),
                (t.intern("clear".into()),  CLEAR.into()),
                (t.intern("name".into()),   NAME.into()),
                (t.intern("mkname".into()), MKNAME.into()),
                (t.intern("mkstr".into()),  MKSTR.into()),
                (t.intern("mkact".into()),  MKACT.into()),
                (t.intern("mkpass".into()), MKPASS.into()),
                (t.intern("exec".into()),   EXEC.into()),
                (t.intern("mark".into()),   MARK.into()),
                (t.intern("mklist".into()), MKLIST.into()),
            ])
        }
    }

    pub fn get(&self, string: String) -> Option<Frame> {
        self.dict.get::<String>(&string).cloned()
    }

    pub fn insert(&mut self, name: Name, frame: Frame) {
        self.dict.insert(name, frame);
    }
}

pub struct Vm {
    op_stack: Vec<Frame>,
    exec_stack: Vec<Frame>,
    dict: Dict,
    intern_table: InternTable,
}

impl Vm {
    pub fn new() -> Self {
        let mut intern_table = InternTable::new();
        Vm {
            op_stack: Vec::new(),
            exec_stack: Vec::new(),
            dict: Dict::new(&mut intern_table),
            intern_table: intern_table,
        }
    }

    pub fn intern(&mut self, string: String) -> Name {
        self.intern_table.intern(string)
    }

    fn exec_op<T: Op>(&mut self, op: T) -> Result<(), Error> {
        match op.exec(self) {
            Some(e) => e.into(),
            None => Ok(()),
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
                Frame::Active(Active::Name(name)) => {
                    let Some(f) = self.dict.get(name.to_string()) else {
                        return Error::Unknown(name.to_string()).into()
                    };
                    self.exec_stack.push(f)
                },

                Frame::Active(Active::String(string)) => {
                    let reader = &mut Reader::new(self);
                    match term::exec_string(reader, string) {
                        None => return Err(Error::Quit),
                        Some(Err(err)) => return Err(err),
                        Some(Ok(())) => (),
                    }
                },
                
                Frame::UnaryOp(op)  => self.exec_op(op)?,
                Frame::BinaryOp(op) => self.exec_op(op)?,
                Frame::StackOp(op)  => self.exec_op(op)?,
                Frame::VmOp(op)     => self.exec_op(op)?,

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
        let vm = &mut Vm::new();

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

        let mut reader = Reader::new(vm);
        let Some(frames) = reader.result(String::from("1 2 add 3 sub")) else {
            panic!("Result: read error")
        };
        let None = vm.result(frames) else {
            panic!("Result: error")
        };
    }

    #[test]
    fn exec() {
        let vm = &mut Vm::new();
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

        let mut reader = Reader::new(vm);
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
        let vm = &mut Vm::new();
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

        let mut reader = Reader::new(vm);
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
