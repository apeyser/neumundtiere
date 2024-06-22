use std::io;
use std::io::Write;

use super::*;
use rpn::Vm;
use reader::Reader;

pub struct Lines {
    lines: std::io::Lines<std::io::StdinLock<'static>>
}

impl Lines {
    pub fn new() -> Self {
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

pub fn exec(vm: &mut Vm, reader: &Reader) -> MainResult {
    super::exec(vm, reader, Lines::new())
}
