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
    match vm.result(cmd) {
        None => (),
        Some(_) => return,
    };

    match Reader::parse("1 2 add 3 sub") {
        Err(e) => {
            println!("Read failure {:?}", e);
            return;
        },
        Ok(frames) => {
            match vm.result(frames) {
                None => (),
                Some(_) => return,
            }
        },
    };
}
