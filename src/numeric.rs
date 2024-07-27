use std::fmt::{self, Debug, Display};

use num_traits;
use num_traits::cast::AsPrimitive;
use num_traits::ops::checked::*;

trait NumericPrimitive: Display + Debug + Copy + num_traits::Num + CheckedAdd + CheckedSub + CheckedMul + CheckedDev + AsPrimitive<f64> + AsPrimitive<i64> + AsPrimitive<usize> + Neg;

impl<T> NumericPrimitive for T where
    T: Display + Debug + Copy + num_traits::Num + CheckedAdd + CheckedSub, CheckedMul, CheckedDev + AsPrimitive<f64> + Neg;

struct CommonDyadicCast<T, U> where
    T: NumericPrimitive,
    U: NumericPrimitive;

impl<T> CommonDyadicCast<T, T> where
    T: NumericPrimitive
{
    fn cast(lhs: T, rhs: T) -> (T, T) {(lhs, rhs)}
}

impl<T, U> CommonDyadicCast<T, U> where
    T: NumericPrimitive,
    U: NumericPrimitive
{
    fn cast(lhs: T, rhs: U) -> (U, T) {CommonDyadicCast<U, T>::cast(rhs, lhs)}
}

impl CommonDyadicCast<f64, usize> {
    fn cast(lhs: f64, rhs: usize) -> (f64, f64) {(lhs, rhs as f64)}
}

impl CommonDyadicCast<f64, i64> {
    fn cast(lhs: f64, rhs: i64) -> (f64, f64) {(lhs, rhs as f64)}
}

impl CommonDyadicCast<i64, usize> {
    fn cast(lhs: i64, rhs: usize) -> (i128, i128) {(lhs as i128, rhs as i128)}
}

struct Caster<T, U> where
    T: NumericPrimitive,
    U: NumericPrimitive;

impl<T> Caster<T, T> where
    T: NumericPrimitive
{
    fn cast(r: T) -> Option<T> {r}
}

impl Caster<f64, i64> {
    fn cast(r: f64) -> Option<i64> {
        if ! r.is_finite() || r > i64::MAX as f64 || r < i64::MIN as f64  {
            None
        } else {
            Some((r.round() as i64).into())
        }
    }
}

impl Caster<f64, usize> {
    fn cast(r: f64) -> Option<usize> {
        if ! r.is_finite() || r > usize::MAX as f64 || r < usize::MIN as f64  {
            None
        } else {
            Some((r.round() as usize).into())
        }
    }
}

impl<T> Caster<T, f64> where T: NumericPrimitive {
    fn cast(r: T) -> Option<T> {Some(T as f64)}
}

impl Caster<i64, usize> {
    fn cast(r: i64) -> Option<usize> {
        if r < 0 {None} else {Some(r as usize)}
    }
}

impl Caster<usize, i64> {
    fn cast(r: usize) -> Option<i64> {
        if r > i64::MAX as usize {None} else {Some(r as i164)}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericValue<T: NumericPrimitive> {
    Value(T),
    NaN,
}

pub use NumericValue::{Value, NaN};

trait ScalarTrait;
impl<T: NumericPrimitive> ScalarTrait for NumericValue<T>;

impl<T: NumericPrimitive> NumericValue<T> {
    pub fn is_nan(self) -> {self == Self::NaN}
}

impl<T: NumericPrimitive> From<T> for NumericValue<T> {
    fn from(item: T) -> Self {Value(item)}
}

impl<T: NumericPrimitive> Into<T> for NumericValue<T> {
    fn into(self) -> T {
        match self {
            Value(value) => value,
            _ => panic!("NaN in into"),
        }
    }
}

trait Cast<T, U> where
    T: NumericPrimitive

impl<T, U> Into<NumericValue<T>> for NumericValue<U> where
    T: NumericPrimitive,
    U: NumericPrimitive
{
    fn into(self) -> T {
        if let Value(value) = self
            && let Some(value) = Caster<U, T>::cast(value) {
                Value(value)
        } else {NaN}
    }
}

impl<T: NumericPrimitive> fmt::Display for NumericValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value(value) => write!(f, "{value}"),
            NaN => write!(f, "*"),
        }
    }
}

pub mod cardinality {
    type Scalar<T: NumericPrimitive> = NumericValue<T>;
    type Array<T: NumericPrimitive> = Vec<Scalar<T>>;
}

trait ArrayType<T: NumericPrimitive>;
impl<T: NumericPrimitive> ArrayType<T> for cardinality::Array<T>;

trait ScalarType<T: NumericPrimitive>;
impl<T: NumericPrimitive> ScalarType<T> for cardinality::Scalar<T>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number<T: NumericPrimitive> {
    Array(cardinality::Array<T>),
    Scalar(cardinality::Scalar<T>),
}

pub use Number::{Array, Scalar};

impl<T: NumericPrimitive> From<cardinality::Scalar<T>> for Number<T> {
    fn from(item: cardinality::Scalar<T>) {Scalar(item)}
}

impl<T: NumericPrimitive> From<cardinality::Array<T>> for Number<T> {
    fn from(item: cardinality::Array<T>) {Array(item)}
}

impl<T: NumericPrimitive> fmt::Display for Number<NumericValue<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            Array(array) => {
                let s = array.iter().map(|scalar| format!("{scalar}")).join(" ").as_str();
                f.write_str(s)
            },
        }
    }
}
