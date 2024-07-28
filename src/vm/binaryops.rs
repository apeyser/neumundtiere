use super::optypes::BinaryOp;

pub const ADD: BinaryOp = BinaryOp::new("add", |a, b| a+b);

pub const SUB: BinaryOp = BinaryOp::new("sub", |a, b| a-b);

pub const MUL: BinaryOp = BinaryOp::new("mul", |a, b| a*b);

pub const DIV: BinaryOp = BinaryOp::new("div", |a, b| a/b);

