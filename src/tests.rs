use crate::vm::*;
use crate::reader::Reader;
use crate::numeric::{Value, Scalar};

fn int_frame(i: i64) -> Frame {
    Num::Int(Scalar(Value(i))).into()
}

#[test]
fn result() {
    let vm = &mut Vm::new();
    
    let cmd: Vec<Frame> = vec![
        int_frame(3),
        int_frame(1),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        int_frame(2),
        binaryops::SUB.into(),
        unaryops::NEG.into(),
    ];
    let None = vm.result(cmd) else {
        panic!("Result: error")
    };

    let mut reader = Reader::new(vm);
    let Some(frames) = reader.result(String::from("1 2 add 3 sub")) else {
        panic!("Result: read error")
    };
    let None = vm.result(frames) else {
        panic!("Result: error")
    };
}

#[test]
fn exec() {
    let vm = &mut Vm::new();
    let frames: Vec<Frame> = vec![
        int_frame(3),
        int_frame(1),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        int_frame(2),
        binaryops::SUB.into(),
        unaryops::NEG.into(),
    ];
    match vm.exec(frames) {
        Err(e) => panic!("Error {e:?}"),
        Ok(None) => panic!("Empty stack"),
        Ok(Some(frame)) => assert_eq!(int_frame(6), frame),
    };

    let mut reader = Reader::new(vm);
    match reader.parse(String::from("1 2 add 4 sub")) {
        Err(e)  => panic!("Parse error: {e:?}"),
        Ok(frames) => match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(int_frame(-1), frame),
        },
    };
}

#[test]
fn stacked() {
    let vm = &mut Vm::new();
    let frames: Vec<Frame> = vec![
        int_frame(3),
        naryops::DUP.into(),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        int_frame(2),
        binaryops::SUB.into(),
        unaryops::NEG.into(),
    ];
    match vm.exec(frames) {
        Err(e) => panic!("Error {e:?}"),
        Ok(None) => panic!("Empty stack"),
        Ok(Some(frame)) => assert_eq!(int_frame(8), frame),
    };

    let mut reader = Reader::new(vm);
    match reader.parse(String::from("1 dup add 4 pop dup sub")) {
        Err(e)  => panic!("Parse error: {e:?}"),
        Ok(frames) => match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(int_frame(0), frame),
        },
    };
}
