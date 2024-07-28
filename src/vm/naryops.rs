use itertools::Itertools;

use super::*;
use crate::error::Error;
use super::optypes::NaryOp;
use crate::numeric::{Scalar, Value, NumericValue};
use crate::numeric::primitive::NumericPrimitive;

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

pub fn to_index<T>(value: NumericValue<T>) -> Result<usize, Error> where
    T: NumericPrimitive
{
    if value.is_nan() {return Error::IllNan.into()};
    let value = value.to_primitive();
    if value < T::zero() {return Error::IllNeg.into()};
    Ok(value.as_())
}

pub fn from_num(num: Num) -> Result<usize, Error> {
    let index = match num {
        Num::Int(Scalar(i))  => to_index(i)?,
        Num::USize(Scalar(i)) => to_index(i)?,
        _ => return Error::OpType.into(),
    };
    Ok(index)
}

fn fget(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (Frame::Passive(Passive::List(ref list)), Frame::Num(n))
        = substack.into_iter().collect_tuple().unwrap()
    else {return Error::OpType.into()};
    
    Ok(vec![list.get(from_num(n)?)?])
}
pub const GET: NaryOp = NaryOp::new("get", fget, 2);

fn fput(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (f1, Frame::Passive(Passive::List(ref mut list)), Frame::Num(n))
        = substack.into_iter().collect_tuple().unwrap()
    else {return Error::OpType.into()};
    
    if let Some(err) = list.put(from_num(n)?, f1) {
        return err.into()
    };
    Ok(vec![])
}
pub const PUT: NaryOp = NaryOp::new("put", fput, 3);

fn flength(mut substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::List(ref mut list)) = substack.pop().unwrap() else {
        return Error::OpType.into()
    };

    Ok(vec![Num::USize(Scalar(Value(list.len()?))).into()])
}
pub const LENGTH: NaryOp = NaryOp::new("length", flength, 1);

fn fgetinterval(substack: Vec<Frame>) -> Result<Vec<Frame>, Error> {
    let (Frame::Passive(Passive::List(ref list)),
            Frame::Num(start),
            Frame::Num(len))
        = substack.into_iter().collect_tuple().unwrap()
    else {return Error::OpType.into()};

    Ok(vec![Passive::List(list.range(from_num(start)?, from_num(len)?)?).into()])
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
