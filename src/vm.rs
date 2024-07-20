use std::fmt;
use std::convert::From;
use std::collections::HashMap;

use super::term;
use super::error::*;
use super::reader::Reader;
use super::save::Save;
use super::list::List;
use super::optypes::*;
use super::vminfo::{self, Vminfo};
use super::optypes;
use super::vmops;
use super::unaryops;
use super::binaryops;
use super::naryops;
use super::stackops;

pub use super::name::{Name, InternTable};
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

type DictBase = HashMap<Name, Frame>;

pub struct Dict {
    dict: DictBase,
}

impl Dict {
    pub fn new(t: &mut InternTable) -> Self {
        let mut dict = HashMap::from_iter([
            &unaryops::NEG as &dyn optypes::Op,
            &binaryops::ADD,
            &binaryops::SUB,
            &binaryops::MUL,
            &binaryops::DIV,
            &stackops::CLEAR,
            &stackops::SHOW,
            &stackops::PEEK,
            &vmops::NAME,
            &vmops::MKNAME,
            &vmops::EXEC,
            &vmops::MKLIST,
            &vmops::LIST,
            &naryops::POP,
            &naryops::DUP,
            &naryops::EXCH,
            &naryops::GET,
            &naryops::PUT,
            &naryops::LENGTH,
            &naryops::GETINTERVAL,
            &naryops::QUIT,
            &naryops::MKSTR,
            &naryops::MKPASS,
            &naryops::MKACT,
            &vminfo::VMSTATUS,
        ].into_iter()
         .map(|op| op.mkpair(t))
        );

        dict.extend([
            ("^",    unaryops::NEG.into()),
            ("+",    binaryops::ADD.into()),
            ("-",    binaryops::SUB.into()),
            ("×",    binaryops::MUL.into()),
            ("÷",    binaryops::DIV.into()),
            ("*",    Num::NaN.into()),
            ("==",   stackops::SHOW.into()),
            ("=",    stackops::PEEK.into()),
            ("mark", Passive::Mark.into()),
            ("null", Frame::Null),
        ].into_iter()
          .map(|(s, f)| (t.intern(s.into()), f))
        );
        
        Dict {dict}
    }

    pub fn get(&self, string: String) -> Option<Frame> {
        self.dict.get::<String>(&string).cloned()
    }

    pub fn insert(&mut self, name: Name, frame: Frame) {
        self.dict.insert(name, frame);
    }
}

pub struct Vm {
    pub(crate) op_stack: Vec<Frame>,
    pub(crate) exec_stack: Vec<Frame>,
    pub(crate) save: Save,
    pub(crate) dict: Dict,
    pub(crate) intern_table: InternTable,
    pub(crate) vminfo: Vminfo,
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
            vminfo: Vminfo::new(),
        }
    }

    pub fn intern(&mut self, string: String) -> Name {
        self.intern_table.intern(string)
    }

    fn exec_op<T: optypes::Op>(&mut self, op: T) -> Result<(), Error> {
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

    pub fn stack(&self) -> &Vec<Frame> {
        &self.op_stack
    }

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
