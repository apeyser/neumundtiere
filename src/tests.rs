use super::vm::*;
use super::reader::Reader;
use super::unaryops;
use super::binaryops;
use super::naryops;

#[test]
fn result() {
    let vm = &mut Vm::new();
    
    let cmd: Vec<Frame> = vec![
        Num::Int(3).into(),
        Num::Int(1).into(),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        Num::Int(2).into(),
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
        Num::Int(3).into(),
        Num::Int(1).into(),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        Num::Int(2).into(),
        binaryops::SUB.into(),
        unaryops::NEG.into(),
    ];
    match vm.exec(frames) {
        Err(e) => panic!("Error {e:?}"),
        Ok(None) => panic!("Empty stack"),
        Ok(Some(frame)) => assert_eq!(Frame::Num(6.into()), frame),
    };

    let mut reader = Reader::new(vm);
    match reader.parse(String::from("1 2 add 4 sub")) {
        Err(e)  => panic!("Parse error: {e:?}"),
        Ok(frames) => match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Num((-1).into()), frame),
        },
    };
}

#[test]
fn stacked() {
    let vm = &mut Vm::new();
    let frames: Vec<Frame> = vec![
        Num::Int(3).into(),
        naryops::DUP.into(),
        binaryops::ADD.into(),
        unaryops::NEG.into(),
        Num::Int(2).into(),
        binaryops::SUB.into(),
        unaryops::NEG.into(),
    ];
    match vm.exec(frames) {
        Err(e) => panic!("Error {e:?}"),
        Ok(None) => panic!("Empty stack"),
        Ok(Some(frame)) => assert_eq!(Frame::Num(8.into()), frame),
    };

    let mut reader = Reader::new(vm);
    match reader.parse(String::from("1 dup add 4 pop dup sub")) {
        Err(e)  => panic!("Parse error: {e:?}"),
        Ok(frames) => match vm.exec(frames) {
            Err(e) => panic!("Error {e:?}"),
            Ok(None) => panic!("Empty stack"),
            Ok(Some(frame)) => assert_eq!(Frame::Num(0.into()), frame),
        },
    };
}

