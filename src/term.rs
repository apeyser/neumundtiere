pub mod line;
pub mod readline;
pub mod string;

use super::error::*;
use super::reader::{self, Reader};
use super::MainResult;

pub fn exec_string(reader: &mut Reader, string: String) -> Option<Result<(), Error>>
{
    let frames = match reader.parse(string) {
        Ok(frames) => frames,
        Err(err) => return Some(Err(err)),
    };
    
    match reader.exec(frames) {
        Ok(_) => Some(Ok(())),
        Err(err) => {
            match err {
                Error::Quit => None,
                err => Some(Err(err)),
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
            Some(Err(err)) => eprintln!("Error -- {err}"),
            None => return Ok(()),
        }
    };
    Ok(())
}
