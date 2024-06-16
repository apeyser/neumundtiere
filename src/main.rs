mod rpn;
use rpn::*;

fn main() {
    let mut vm = Vm::new();
    let v: Vec<Frame> = vec![3.into(), 1.into(), ADD.into(), NEG.into()];
    match vm.exec(v) {
        Ok(Some(f)) => println!("Result: {}", f),
        Ok(None) => println!("Empty stack"),
        Err(e) => println!("Error: {:?}", e)
    };
}
