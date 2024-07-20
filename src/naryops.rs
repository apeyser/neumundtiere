use itertools::Itertools;

use super::vm::*;
use super::error::Error;
use super::optypes::NaryOp;

fn fpop(_: Vec<Frame>) -> Result<Vec<Frame>, Error>
{
    Ok(vec![])
}
pub const POP: NaryOp = NaryOp::new("pop", fpop, 1);

fn fdup(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (last,) = substack.into_iter().collect_tuple().unwrap();
    let copy = last.clone();
    Ok(vec![last, copy])
}
pub const DUP: NaryOp = NaryOp::new("dup", fdup, 1);

fn fexch(substack: Vec<Frame>) -> Result<Vec<Frame>, Error>
{
    let (f1, f2) = substack.into_iter().collect_tuple().unwrap();
    Ok(vec![f2, f1])
}
pub const EXCH: NaryOp = NaryOp::new("exch", fexch, 2);

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
pub const GET: NaryOp = NaryOp::new("get", fget, 2);

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
pub const PUT: NaryOp = NaryOp::new("put", fput, 3);

fn flength(mut substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::List(ref mut list)) = substack.pop().unwrap() else {
        return Error::OpType.into()
    };

    Ok(vec![Num::Int(list.len()? as i64).into()])
}
pub const LENGTH: NaryOp = NaryOp::new("length", flength, 1);

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
pub const GETINTERVAL: NaryOp = NaryOp::new("getinterval", fgetinterval, 3);

fn fquit(_: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    Error::Quit.into()
}
pub const QUIT: NaryOp = NaryOp::new("quit", fquit, 0);

fn mkstr(mut stack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::Name(name)) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    
    Ok(vec![Frame::Passive(name.into())])
}
pub const MKSTR: NaryOp = NaryOp::new("mkname", mkstr, 1);

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
pub const MKPASS: NaryOp = NaryOp::new("mkpass", mkpass, 1);

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
pub const MKACT: NaryOp = NaryOp::new("mkact", mkact, 1);
