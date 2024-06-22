mod rpn;
use rpn::*;

mod reader;
use reader::*;

use std::io;
use std::io::Write;

struct Lines {
    lines: std::io::Lines<std::io::StdinLock<'static>>
}

impl Lines {
    fn new() -> Self {
        Self {lines: io::stdin().lines()}
    }
}

impl Iterator for Lines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        print!(">> ");
        io::stdout().flush().unwrap();
        match self.lines.next() {
            None => None,
            Some(Err(err)) => panic!("IO Err: {err:?}"),
            Some(Ok(line)) => Some(line),
        }
    }
}

enum Error {
    Reader(reader::Error),
    Rpn(rpn::Error),
    Quit,
}

impl From<reader::Error> for Error {
    fn from(e: reader::Error) -> Error {Error::Reader(e)}
}

impl From<rpn::Error> for Error {
    fn from(e: rpn::Error) -> Error {
        match e {
            rpn::Error::Quit => Error::Quit,
            _ => Error::Rpn(e),
        }
    }
}

fn exec_line(vm: &mut Vm, reader: &Reader, string: &str) -> Result<(), Error> {
    vm.exec(reader.parse(string)?)?;
    Ok(())
}

fn main() {
    let mut vm = Vm::new();
    let reader = Reader::new();
    for line in Lines::new() {
        match exec_line(&mut vm, &reader, &line) {
            Ok(_) => (),
            Err(Error::Quit) => break,
            Err(Error::Reader(err)) => println!("Parse error: {err:?}"),
            Err(Error::Rpn(err)) => println!("Exec error: {err:?}"),
        }
    }
    println!("Quitting with stack {:?}", vm.stack());
}
