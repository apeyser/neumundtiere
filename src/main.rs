mod rpn;
use rpn::*;

mod reader;
use reader::*;

fn main() {
    let mut vm = Vm::new();
    
    let cmd: Vec<Frame> = vec![
        3.into(),
        1.into(),
        ADD.into(),
        NEG.into(),
        2.into(),
        SUB.into(),
        NEG.into(),
    ];
    let None = vm.result(cmd) else {
        return
    };

    let reader = Reader::new();
    let Some(frames) = reader.result("1 2 add 3 sub") else {
        return
    };
    let None = vm.result(frames) else {
        return
    };
}
