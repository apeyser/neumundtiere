use super::optypes::UnaryOp;

pub const NEG: UnaryOp = UnaryOp::new("neg", |a| -a);
