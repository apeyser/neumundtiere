pub use numeric_ops::*;

trait Neg: NumericPrimitive {
    fn neg(self) -> Result<NumericValue<Self>, Error>;
}

impl Neg for i64 {
    fn neg(self) -> Result<NumericValue<Self>, Error> {Ok((-self).into())}
}

impl Neg for usize {
    fn neg(self) -> Result<NumericValue<Self>, Error> {Err::NegRangeUsize}
}

impl Neg for f64 {
    fn neg(self) -> Result<NumericValue<Self>, Error> {Ok((-self).into())}
}

struct NegOp;
impl MonadicOp for NegOp {
    fn op<T>(val: T) -> Result<NumericValue<T>, Error> where
        T: NumericPrimitive
    {<val as Neg>.neg()}
}

struct CosOp;
impl FloatMonadicOp for CosOp {
    fn floatop(val: f64) -> f64 {val.cos()}
}

struct AddOp;
impl DyadicOp for AddOp {
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if Some(value) = lhs.checked_add(rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

struct SubOp;
impl DyadicOp for SubOp {
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if Some(value) = lhs.checked_sub(rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

struct MulOp;
impl DyadicOp for MulOp {
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if Some(value) = lhs.checked_mul(rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

struct DivOp;
impl DyadicOp for DivOp {
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if Some(value) = lhs.checked_div(rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}
