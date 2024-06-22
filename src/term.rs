pub mod line;
pub mod readline;
pub mod string;

use super::rpn::{self, Vm};
use super::reader::{self, Reader};
use super::MainResult;

pub fn exec_string(vm: &mut Vm, reader: &Reader, string: String) -> Option<MainResult>
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

pub fn exec<T>(vm: &mut Vm, reader: &Reader, lines: T) -> MainResult
    where T: Iterator<Item=String>
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
