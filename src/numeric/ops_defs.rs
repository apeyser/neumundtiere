use crate::error::*;

use super::*;
use super::ops::*;
use super::primitive::NumericPrimitive;

pub struct NegOp;
impl MonadicOp for NegOp {
    fn func<T>(val: T) -> Result<NumericValue<T>, Error> where
        T: NumericPrimitive {
        if let Some(value) = val.checked_neg() {Ok(Value(value))} else {Ok(NaN)}
    }
}

pub struct CosOp;
impl FloatMonadicOp for CosOp {
    fn float_func(val: f64) -> Result<Option<f64>, Error> {Ok(Some(val.cos()))}
}

pub struct AddOp;
impl DyadicOp for AddOp {
    fn func<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if let Some(value) = lhs.checked_add(&rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

pub struct SubOp;
impl DyadicOp for SubOp {
    fn func<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if let Some(value) = lhs.checked_sub(&rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

pub struct MulOp;
impl DyadicOp for MulOp {
    fn func<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if let Some(value) = lhs.checked_mul(&rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}

pub struct DivOp;
impl DyadicOp for DivOp {
    fn func<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive {
        if let Some(value) = lhs.checked_div(&rhs) {Ok(Value(value))} else {Ok(NaN)}
    }
}
