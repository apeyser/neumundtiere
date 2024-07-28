use itertools::Itertools;

use super::*;
use crate::error::Error;
use super::optypes::VmOp;
use super::naryops::from_num;

fn name_op(stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let (frame, Frame::Passive(Passive::Name(name)))
        = stack.into_iter().collect_tuple().unwrap()
    else {
        return Error::OpType.into()
    };
    
    let Some(dict) = vm.dict_stack.front_mut() else {
        panic!("dict_stack empty")
    };
    dict.put(name, frame);
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
    let mark: Frame = Passive::Mark.into();
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

    let Some(csave) = vm.save_stack.last_mut() else {
        panic!("save stack is empty")
    };
    let list = csave.put(r)?;
    Ok(vec![Passive::List(list).into()])
}
pub const MKLIST: VmOp = VmOp::new("mklist", mklist, 0);

fn mkproc(_: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let mark: Frame = Active::Mark.into();
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

    let Some(csave) = vm.save_stack.last_mut() else {
        panic!("save stack is empty")
    };
    let list = csave.put(r)?;
    assert!(vm.proc_depth > 0);
    vm.proc_depth -= 1;
    Ok(vec![Active::List(list).into()])
}
pub const MKPROC: VmOp = VmOp::new("mkproc", mkproc, 0);

fn flist(mut stack: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let Frame::Num(n) = stack.pop().unwrap()
    else {
        return Error::OpType.into()
    };

    let index = from_num(n)?; 
    let Some(csave) = vm.save_stack.last_mut() else {
        panic!("save stack is empty")
    };
    let list = csave.put(vec![Frame::Null; index])?;
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
