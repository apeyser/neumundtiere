use itertools::Itertools;

use super::vm::*;
use super::error::Error;

fn name_op(stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let (frame, Frame::Passive(Passive::Name(name)))
        = stack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };
    
    vm.dict.insert(name, frame);
    Ok(vec![])
}
pub const NAME: VmOp = VmOp::new("name", name_op, 2);

fn mkname(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let Frame::Passive(Passive::String(string)) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };

    let name = vm.intern_table.intern(string);
    Ok(vec![Passive::Name(name).into()])
}
pub const MKNAME: VmOp = VmOp::new("mkname", mkname, 1);

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
pub const MKLIST: VmOp = VmOp::new("mklist", mklist, 0);

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
pub const LIST: VmOp = VmOp::new("list", flist, 1);

fn op_exec(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let frame@Frame::Active(_) = stack.pop().unwrap() else {
        return Error::OpType.into()
    };
    
    vm.exec_stack.push(frame);
    Ok(vec![])
}
pub const EXEC: VmOp = VmOp::new("exec", op_exec, 1);
