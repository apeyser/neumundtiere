use super::*;
use rpn::Vm;
use reader::Reader;

pub fn exec(vm: &mut Vm, reader: &Reader, string: String) {
    exec_string(vm, reader, string);
    println!("Quitting with stack {:?}", vm.stack());
}
