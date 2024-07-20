use std::fmt;
use std::convert::From;
use std::collections::HashMap;

use itertools::Itertools;

use super::term;
use super::error::*;
use super::reader::Reader;
use super::name::{Name, InternTable};
use super::save::Save;
use super::list::List;

pub use super::num::Num;

#[derive(Debug, Clone, PartialEq)]
pub enum Active {
    String(String),
    Name(Name),
    Mark,
    List(List),
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

impl From<List> for Active {
    fn from(item: List) -> Self {
        Active::List(item)
    }
}

impl fmt::Display for Active {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Active::String(string) => write!(f, "~({string})"),
            Active::Name(name) => write!(f, "~/({name})"),
            Active::Mark => write!(f, "{{"),
            Active::List(list) => write!(f, "{{ {list} }}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Passive {
    String(String),
    Name(Name),
    Mark,
    List(List),
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

impl From<List> for Passive {
    fn from(item: List) -> Self {
        Passive::List(item)
    }
}

impl fmt::Display for Passive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Passive::String(string) => write!(f, "({string})"),
            Passive::Name(name) => write!(f, "/({name})"),
            Passive::Mark => write!(f, "["),
            Passive::List(list) => write!(f, "[ {list} ]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Num(Num),
    Null,
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    StackOp(StackOp),
    NaryOp(NaryOp),
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

impl From<NaryOp> for Frame {
    fn from(item: NaryOp) -> Self {
        Frame::NaryOp(item)
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
            Frame::Null           => write!(f, "null"),
            Frame::Num(v)         => write!(f, "{v}"),
            Frame::StackOp(op)    => write!(f, "{op}"),
            Frame::VmOp(op)       => write!(f, "{op}"),
            Frame::UnaryOp(op)    => write!(f, "{op}"),
            Frame::BinaryOp(op)   => write!(f, "{op}"),
            Frame::NaryOp(op)     => write!(f, "{op}"),
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

type StackOpFunc = fn(&Vec<Frame>, Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>;
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

impl fmt::Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn fclear(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Ok((vec![], stack.len()))
}
pub const CLEAR: StackOp = StackOp {
    name: "clear",
    op: fclear,
    n: 0,
};

fn fshow(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    println!("Stack:");
    for v in stack.into_iter().rev() {
        println!("  {v}");
    };
    println!("-----");
    Ok((vec![], 0))
}
pub const SHOW: StackOp = StackOp {
    name: "show",
    op: fshow,
    n: 0,
};

fn fpeek(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    match stack.last() {
        None => println!("Stack: empty"),
        Some(frame) => println!("Top: {frame}"),
    };
    Ok((vec![], 0))
}
pub const PEEK: StackOp = StackOp {
    name: "peek",
    op: fpeek,
    n: 0,
};

type NaryOpFunc = fn(stack: Vec<Frame>) -> Result<Vec<Frame>, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NaryOp {
    name: &'static str,
    op: NaryOpFunc,
    n: usize,
}

impl Op for NaryOp {
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

impl fmt::Display for NaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn fpop(_: Vec<Frame>) -> Result<Vec<Frame>, Error>
{
    Ok(vec![])
}
pub const POP: NaryOp = NaryOp {
    name: "pop",
    op: fpop,
    n: 1,
};

fn fdup(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (last,) = substack.into_iter().collect_tuple().unwrap();
    let copy = last.clone();
    Ok(vec![last, copy])
}
pub const DUP: NaryOp = NaryOp {
    name: "dup",
    op: fdup,
    n: 1,
};

fn fexch(substack: Vec<Frame>) -> Result<Vec<Frame>, Error>
{
    let (f1, f2) = substack.into_iter().collect_tuple().unwrap();
    Ok(vec![f2, f1])
}
pub const EXCH: NaryOp = NaryOp {
    name: "exch",
    op: fexch,
    n: 2,
};

fn fget(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (Frame::Passive(Passive::List(ref list)), Frame::Num(Num::Int(i)))
        = substack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };

    if i < 0 {
        return Error::IllNeg.into()
    };
    
    Ok(vec![list.get(i as usize)?])
}
pub const GET: NaryOp = NaryOp {
    name: "get",
    op: fget,
    n: 2,
};

fn fput(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (f1,
         Frame::Passive(Passive::List(ref mut list)),
         Frame::Num(Num::Int(i)))
        = substack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };

    if i < 0 {
        return Error::IllNeg.into()
    };

    if let Some(err) = list.put(i as usize, f1) {
        return err.into()
    };

    Ok(vec![])
}
pub const PUT: NaryOp = NaryOp {
    name: "put",
    op: fput,
    n: 3,
};

fn flength(mut substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::List(ref mut list)) = substack.pop().unwrap() else {
        return Error::OpType.into()
    };

    Ok(vec![Num::Int(list.len()? as i64).into()])
}
pub const LENGTH: NaryOp = NaryOp {
    name: "length",
    op: flength,
    n: 1,
};

fn fgetinterval(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (Frame::Passive(Passive::List(ref list)),
         Frame::Num(Num::Int(start)),
         Frame::Num(Num::Int(len)))
        = substack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };

    if start < 0 || len < 0 {
        return Error::IllNeg.into();
    };

    Ok(vec![Passive::List(list.range(start as usize, len as usize)?).into()])
}
pub const GETINTERVAL: NaryOp = NaryOp {
    name: "getinterval",
    op: fgetinterval,
    n: 3,
};

fn fquit(_: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    Error::Quit.into()
}
pub const QUIT: NaryOp = NaryOp {
    name: "quit",
    op: fquit,
    n: 0,
};


fn mkstr(mut stack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::Name(name)) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    
    Ok(vec![Frame::Passive(name.into())])
}
pub const MKSTR: NaryOp = NaryOp {
    name: "mkname",
    op: mkstr,
    n: 1,
};

fn mkpass(mut stack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Active(active) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    let passive = match active {
        Active::Name(name)     => Passive::Name(name),
        Active::String(string) => Passive::String(string),
        Active::Mark           => Passive::Mark,
        Active::List(list)     => Passive::List(list),
    };
    Ok(vec![passive.into()])
}
pub const MKPASS: NaryOp = NaryOp {
    name: "mkpass",
    op: mkpass,
    n: 1,
};

fn mkact(mut stack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(passive) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    let active = match passive {
        Passive::Name(name)     => Active::Name(name),
        Passive::String(string) => Active::String(string),
        Passive::Mark           => Active::Mark,
        Passive::List(list)     => Active::List(list),
    };
    Ok(vec![active.into()])
}
pub const MKACT: NaryOp = NaryOp {
    name: "mkact",
    op: mkact,
    n: 1,
};

type VmOpFunc = fn(stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error>;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VmOp {
    name: &'static str,
    op: VmOpFunc,
    n: usize,
}

impl Op for VmOp {
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

impl fmt::Display for VmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn name_op(stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let (frame, Frame::Passive(Passive::Name(name)))
        = stack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };
    
    vm.dict.insert(name, frame);
    Ok(vec![])
}
pub const NAME: VmOp = VmOp {
    name: "name",
    op: name_op,
    n: 2,
};

fn mkname(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::String(string)) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };

    let name = vm.intern_table.intern(string);
    Ok(vec![Passive::Name(name).into()])
}
pub const MKNAME: VmOp = VmOp {
    name: "mkname",
    op: mkname,
    n: 1,
};

fn mklist(_: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let mark = Frame::Passive(Passive::Mark);
    let r: Vec<Frame>
        = vm.op_stack.iter().cloned().rev()
               .take_while(|frame| *frame != mark)
               .collect::<Vec<Frame>>().into_iter()
               .rev().collect();
    
    if r.len() == vm.op_stack.len() {
        return Err(Error::StackUnderflow)
    };

    let len = vm.op_stack.len()-r.len()-1;
    vm.op_stack.truncate(len);

    let list = vm.save.intern_list(r);
    Ok(vec![Passive::List(list).into()])
}
pub const MKLIST: VmOp = VmOp {
    name: "mklist",
    op: mklist,
    n: 0,
};

fn flist(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let Frame::Num(Num::Int(n)) = stack.pop().unwrap()
    else {
        return Error::OpType.into()
    };

    if n < 0 {
        return Error::IllNeg.into()
    };

    let list = vm.save.intern_list(vec![Frame::Null; n as usize]);
    Ok(vec![Passive::List(list).into()])
}
pub const LIST: VmOp = VmOp {
    name: "list",
    op: flist,
    n: 1,
};

fn op_exec(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let frame@Frame::Active(_) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    
    vm.exec_stack.push(frame);
    Ok(vec![])
}
pub const EXEC: VmOp = VmOp {
    name: "exec",
    op: op_exec,
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

        let r = (self.op)(i1, i2);
        stack.push(r.into());
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
                (t.intern("mark".into()),   Passive::Mark.into()),
                (t.intern("null".into()),   Frame::Null),
                (t.intern("mklist".into()), MKLIST.into()),
                (t.intern("list".into()),   LIST.into()),
                (t.intern("get".into()),    GET.into()),
                (t.intern("put".into()),    PUT.into()),
                (t.intern("length".into()), LENGTH.into()),
                (t.intern("getinterval".into()), GETINTERVAL.into()),
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
    save: Save,
    dict: Dict,
    intern_table: InternTable,
}

impl Vm {
    pub fn new() -> Self {
        let mut intern_table = InternTable::new();
        Vm {
            op_stack: Vec::new(),
            exec_stack: Vec::new(),
            save: Save::new(),
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
                        None => return Error::Quit.into(),
                        Some(Err(err)) => return err.into(),
                        Some(Ok(())) => (),
                    }
                },
                
                Frame::UnaryOp(op)  => self.exec_op(op)?,
                Frame::BinaryOp(op) => self.exec_op(op)?,
                Frame::StackOp(op)  => self.exec_op(op)?,
                Frame::VmOp(op)     => self.exec_op(op)?,
                Frame::NaryOp(op)   => self.exec_op(op)?,

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
