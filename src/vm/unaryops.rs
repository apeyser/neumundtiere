use super::optypes::UnaryOp;

pub const NEG: UnaryOp = UnaryOp::new("neg", |a| -a);
pub const COS: UnaryOp = UnaryOp::new("cos", |a| a.cos());
