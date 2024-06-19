mod rpn;
use rpn::*;

mod reader;
use reader::*;

fn check_return(expect: Option<Frame>, got: Result<Option<Frame>, rpn::Error>) {
    match got {
        Err(e) => panic!("Error: {:?}", e),
        Ok(v) => match (expect, v) {
            (None,     None)     => println!("Empty stack expected"),
            (None,     Some(f))  => panic!("Expected empty stack, got {}", f),
            (Some(f),  None)     => panic!("Expected {}, but stack empty", f),
            (Some(f1), Some(f2)) => if f1 == f2 {
                println!("Expected and got {}", f1)
            } else {
                panic!("Expected {}, got {}", f1, f2)
            }
        }
    }
}

fn result(vm: &mut Vm) {
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
        panic!("Result: error")
    };

    let reader = Reader::new();
    let Some(frames) = reader.result("1 2 add 3 sub") else {
        panic!("Result: read error")
    };
    let None = vm.result(frames) else {
        panic!("Result: error")
    };
}

fn exec(vm: &mut Vm) {
    let cmd: Vec<Frame> = vec![
        3.into(),
        1.into(),
        ADD.into(),
        NEG.into(),
        2.into(),
        SUB.into(),
        NEG.into(),
    ];
    check_return(Some(Frame::Int(6)), vm.exec(cmd));

    let reader = Reader::new();
    match reader.parse("1 2 add 4 sub") {
        Err(e)  => panic!("Parse error: {:?}", e),
        Ok(cmd) => check_return(Some(Frame::Int(-1)), vm.exec(cmd)),
    };
}

fn main() {
    let mut vm = Vm::new();
    result(&mut vm);
    exec(&mut vm);
}
