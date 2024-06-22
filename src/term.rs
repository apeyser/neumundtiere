pub mod line;
pub mod readline;
pub mod string;

use super::rpn::{self,Vm};
use super::reader::{self,Reader};

pub enum Error {
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

pub fn exec_string(vm: &mut Vm, reader: &Reader, string: String) -> bool {
    let frames = match reader.parse(string) {
        Ok(frames) => frames,
        Err(err) => {
            println!("Parse error: {err:?}");
            return false
        }
    };
    
    match vm.exec(frames) {            
        Ok(_) => true,
        Err(err) => {
            match err {
                rpn::Error::Quit => (),
                err => println!("Exec error: {err:?}"),
            };
            false
        }
    }
}

pub fn exec<T: Iterator<Item=String>>(vm: &mut Vm, reader: &Reader, lines: T) {
    for line in lines {
        if ! exec_string(vm, reader, line) {
            break
        }
    }
    println!("Quitting with stack {:?}", vm.stack());
}
