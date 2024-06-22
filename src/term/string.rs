use super::*;
use rpn::Vm;
use reader::Reader;

pub fn exec(vm: &mut Vm, reader: &Reader, string: String) -> MainResult {
    exec_string(vm, reader, string).unwrap_or(Ok(()))
}
