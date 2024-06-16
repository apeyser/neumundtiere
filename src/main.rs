mod rpn;
use rpn::*;

fn main() {
    let cmd: Vec<Frame> = vec![
        3.into(),
        1.into(),
        ADD.into(),
        NEG.into(),
        2.into(),
        SUB.into(),
        NEG.into(),
    ];
    match Vm::new().exec(cmd.into_iter()) {
        Ok(Some(f)) => println!("Result: {}", f),
        Ok(None) => println!("Empty stack"),
        Err(e) => println!("Error: {:?}", e)
    };
}
