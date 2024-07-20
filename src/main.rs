pub mod num;
pub mod term;
pub mod vm;
pub mod name;
pub mod reader;
pub mod error;
pub mod list;
pub mod save;
pub mod dict;
pub mod vminfo;
pub mod optypes;
pub mod vmops;
pub mod unaryops;
pub mod naryops;
pub mod binaryops;
pub mod stackops;

#[cfg(test)]
mod tests;

mod run;
fn main() -> run::MainResult {run::run()}
