use std::error::Error;

pub mod line;
pub mod readline;
pub mod string;

use super::rpn::{self, Vm};
use super::reader::{self, Reader};

pub fn exec_string(vm: &mut Vm, reader: &Reader, string: String) -> Option<Result<(), Box<dyn Error>>>
{
    let frames = match reader.parse(string) {
        Ok(frames) => frames,
        Err(err) => return Some(Err(Box::new(err))),
    };
    
    match vm.exec(frames) {
        Ok(_) => Some(Ok(())),
        Err(ref err) => {
            match err {
                rpn::Error::Quit => None,
                err => return Some(Err(Box::new(err.clone()))),
            }
        }
    }
}

pub fn exec<T: Iterator<Item=String>>(vm: &mut Vm, reader: &Reader, lines: T) -> Result<(), Box<dyn Error>>
{
    for line in lines {
        match exec_string(vm, reader, line) {
            Some(Ok(())) => (),
            Some(Err(err)) => println!("Error -- {err}"),
            None => return Ok(()),
        }
    };
    Ok(())
}
