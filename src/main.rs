mod rpn;
use rpn::*;

mod reader;
use reader::*;

use std::fmt;
use std::convert::From;

enum Printer {
    Frame(Frame),
    Empty,
}

impl fmt::Display for Printer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Printer::Frame(frame) => write!(f, "{}", frame),
            Printer::Empty => write!(f, "Empty Stack"),
        }
    }
}

impl From<Option<Frame>> for Printer {
    fn from(item: Option<Frame>) -> Self {
        match item {
            None => Printer::Empty,
            Some(frame) => Printer::Frame(frame),
        }
    }
}

fn check_return(expect: Option<Frame>, got: Result<Option<Frame>, rpn::Error>) {
    match got {
        Err(e) => panic!("Error: {:?}", e),
        Ok(v) => if v == expect {
            println!("Expected and got {}", Printer::from(v))
        } else {
            panic!("Expected {}, got {}", Printer::from(expect), Printer::from(v))
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
