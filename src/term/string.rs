use super::*;
use reader::Reader;

pub fn exec(reader: &mut Reader, string: String) -> MainResult {
    exec_string(reader, string).unwrap_or(Ok(()))
}
