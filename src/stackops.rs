use super::vm::*;
use super::error::Error;
use super::optypes::StackOp;

fn fclear(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    Ok((vec![], stack.len()))
}
pub const CLEAR: StackOp = StackOp::new("clear", fclear, 0);

fn fshow(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error> {
    println!("Stack:");
    for v in stack.into_iter().rev() {
        println!("  {v}");
    };
    println!("-----");
    Ok((vec![], 0))
}
pub const SHOW: StackOp = StackOp::new("show", fshow, 0);

fn fpeek(stack: &Vec<Frame>, _: Vec<Frame>) -> Result<(Vec<Frame>, usize), Error>
{
    match stack.last() {
        None => println!("Stack: empty"),
        Some(frame) => println!("Top: {frame}"),
    };
    Ok((vec![], 0))
}
pub const PEEK: StackOp = StackOp::new("peek", fpeek, 0);
