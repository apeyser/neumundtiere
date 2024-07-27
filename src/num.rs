use std::fmt;
use std::ops::{Add, Sub, Div, Mul, Neg};

use ops_defs::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Num {
    Int(Number<i64>),
    Float(Number<f64>),
    USize(Number<usize>),
}

impl From<i64> for Num {
    fn from(item: i64) -> Self {Num::Int(item.into())}
}

impl From<Vec<i64>> for Num {
    fn from(item: Vec<i64>) -> Self {Num::Int(item.into())}
}

impl From<Number<i64>> for Num {
    fn from(item: Number<i64>) -> Self {Num::Int(item)}
}

impl From<usize> for Num {
    fn from(item: i64) -> Self {Num::USize(item.into())}
}

impl From<Vec<usize>> for Num {
    fn from(item: Vec<usize>) -> Self {Num::USize(item.into())}
}

impl From<Number<usize>> for Num {
    fn from(item: Number<usize>) -> Self {Num::USize(item)}
}

impl From<f64> for Num {
    fn from(item: f64) -> Self {Num::Float(item.into())}
}

impl From<Vec<f64>> for Num {
    fn from(item: Vec<f64>) -> Self {Num::Float(item.into())}
}

impl From<Number<f64>> for Num {
    fn from(item: Number<f64>) -> Self {Num::Float(item)}
}

impl Num {
    pub fn applyMonadic<M: MonadicOp>(self) -> Result<Num, Error> {
        match self {
            Num::Int(i)   => Ok(M::apply(i)?.into()),
            Num::Float(f) => Ok(M::apply(f)?.into()),
            Num::USize(u) => Ok(M::apply(u)?.into()),
        }
    }

    fn applyDyadicTarget<D, T>(lhs: Number<T>, rhs: Num) ->
        Result<Num, Error> where
        T: NumericPrimitive,
        D: DyadicOp
    {
        match rhs {
            Num::Int(rhs)   => Ok(D::apply(lhs, rhs)?.into()),
            Num::Float(rhs) => Ok(D::apply(lhs, rhs)?.into()),
            Num::USize(rhs) => Ok(D::apply(lhs, rhs)?.into()),
        }
    }

    pub fn applyDyadic<D: DyadicOp>(self, rhs: Num) -> Result<Num, Error> {
        match self {
            Num::Int(lhs)   => Self::applyDyadicTarget::<D>(lhs, rhs),
            Num::Float(lhs) => Self::applyDyadicTarget::<D>(lhs, rhs),
            Num::USize(lhs) => Self::applyDyadicTarget::<D>(lhs, rhs),
        }
      }
    }
}

impl Neg for Num {
    type Output = Result<Num, Error>;
    fn neg(self) -> Self::Output {Self::applyMonadic::<NegOp>(self)}
}

impl Num {
    fn cos(self) -> Result<Self, Error> {Self::applyMonadic::<CosOp>(self)}
}

impl Add for Num {
    type Output = Result<Num, Error>;
    fn add(self, rhs: Self) -> Self::Output {Self::applyDyadic::<AddOp>(self, rhs)}
}

impl Sub for Num {
    type Output = Num;
    fn sub(self, rhs: Self) -> Self::Output {Self::applyDyadic::<SubOp>(self, rhs)}
}

impl Mul for Num {
    type Output = Num;
    fn mul(self, rhs: Self) -> Self::Output {Self::applyDyadic::<MulOp>(self, rhs)}
}

impl Div for Num {
    type Output = Num;
    fn div(self, rhs: Self) -> Self::Output {Self::applyDyadic::<DivOp>(self, rhs)}
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Num::Int(i)   => write!(f, "{i}"),
            Num::Float(x) => write!(f, "{x:+e}"),
            Num::USize(i) => write!(f, "{i}"),
        }
    }
}
