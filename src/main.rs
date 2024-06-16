mod rpn;
use rpn::*;

fn main() {
    match Vm::new().exec(
        vec![
            3.into(),
            1.into(),
            ADD.into(),
            NEG.into(),
            2.into(),
            SUB.into(),
            NEG.into(),
        ]
    ) {
        Ok(Some(f)) => println!("Result: {}", f),
        Ok(None) => println!("Empty stack"),
        Err(e) => println!("Error: {:?}", e)
    };
}
