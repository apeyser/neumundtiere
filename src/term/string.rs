use super::*;
use reader::Reader;

pub fn exec(reader: &mut Reader, string: String) -> MainResult {
    match exec_string(reader, string) {
        None|Some(Ok(())) => Ok(()),
        Some(Err(err)) => Err(Box::new(err)),
    }
}
