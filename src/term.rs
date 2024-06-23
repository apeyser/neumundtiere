pub mod line;
pub mod readline;
pub mod string;

use super::rpn;
use super::reader::{self, Reader};
use super::MainResult;

pub fn exec_string(reader: &mut Reader, string: String) -> Option<MainResult>
{
    let frames = match reader.parse(string) {
        Ok(frames) => frames,
        Err(err) => return Some(Err(Box::new(err))),
    };
    
    match reader.exec(frames) {
        Ok(_) => Some(Ok(())),
        Err(ref err) => {
            match err {
                rpn::Error::Quit => None,
                err => return Some(Err(Box::new(err.clone()))),
            }
        }
    }
}

pub fn exec<T>(reader: &mut Reader, lines: T) -> MainResult
    where T: Iterator<Item=String>
{
    for line in lines {
        match exec_string(reader, line) {
            Some(Ok(())) => (),
            Some(Err(err)) => println!("Error -- {err}"),
            None => return Ok(()),
        }
    };
    Ok(())
}
