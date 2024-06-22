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
        Self {
            lines: io::stdin().lines(),
        }
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

fn exec_line(vm: &mut Vm, reader: &Reader, string: &str) -> Option<()> {
    match reader.parse(string) {
        Err(err) => println!("Parse error: {err:?}"),
        Ok(frames) => {
            match vm.exec(frames) {
                Err(rpn::Error::Quit) => {
                    println!("Quiting with stack {:?}", vm.stack());
                    return None
                },
                Err(err) => println!("Exec error: {err:?}, {:?}", vm.stack()),
                Ok(_) => (),
            }
        }
    };
    Some(())
}

fn main() {
    let mut vm = Vm::new();
    let reader = Reader::new();
    for line in Lines::new() {
        if None == exec_line(&mut vm, &reader, &line) {
            break
        }
    }
}
