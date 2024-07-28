use std::fmt;
use std::ops::{Add, Sub, Div, Mul, Neg};

use crate::error::*;
use crate::numeric::{Number, CasterBuilder, CasterBuilderTrait, Caster, CasterTrait, CastFromFloat};
use crate::numeric::ops::*;
use crate::numeric::ops_defs::*;
use crate::numeric::primitive::NumericPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    Int(Number<i64>),
    Float(Number<f64>),
    USize(Number<usize>),
}

impl From<Number<i64>> for Num {
    fn from(item: Number<i64>) -> Self {Num::Int(item)}
}

impl From<Number<f64>> for Num {
    fn from(item: Number<f64>) -> Self {Num::Float(item)}
}

impl From<Number<usize>> for Num {
    fn from(item: Number<usize>) -> Self {Num::USize(item)}
}

impl Num {
    pub fn apply_monadic<M: MonadicOp>(self) -> Result<Num, Error> {
        match self {
            Num::Int(i)   => Ok(M::apply(i)?.into()),
            Num::Float(f) => Ok(M::apply(f)?.into()),
            Num::USize(u) => Ok(M::apply(u)?.into()),
        }
    }

    fn apply_dyadic_strip<D, T, U, C>(lhs: Number<T>, rhs: Number<U>, _caster: C)
        -> Result<Num, Error>
    where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        D: DyadicOp,
        Num: From<Number<T>>,
        C: CasterTrait<T, U> {
        Ok(D::apply::<_, _, C>(lhs, rhs)?.into())
    }

    fn apply_dyadic_target<D, B, T>(lhs: Number<T>, rhs: Num)
        -> Result<Num, Error>
    where
        T: NumericPrimitive + CastFromFloat,
        D: DyadicOp,
        Num: From<Number<T>>,
        B: CasterBuilderTrait<T>,
        Caster::<T, i64>: CasterTrait<T, i64>,
        Caster::<T, f64>: CasterTrait<T, f64>,
        Caster::<T, usize>: CasterTrait<T, usize>,
    {
        match rhs {
            Num::Int(rhs)   => Self::apply_dyadic_strip::<D, _, _, _>(lhs, rhs, B::caster_i64()),
            Num::Float(rhs) => Self::apply_dyadic_strip::<D, _, _, _>(lhs, rhs, B::caster_f64()),
            Num::USize(rhs) => Self::apply_dyadic_strip::<D, _, _, _>(lhs, rhs, B::caster_usize()),
        }
    }

    pub fn apply_dyadic<D: DyadicOp>(self, rhs: Num) -> Result<Num, Error> {
        match self {
            Num::Int(lhs)   =>
                Self::apply_dyadic_target::<D, CasterBuilder::<i64>, _>(lhs, rhs),
            Num::Float(lhs) =>
                Self::apply_dyadic_target::<D, CasterBuilder::<f64>, _>(lhs, rhs),
            Num::USize(lhs) =>
                Self::apply_dyadic_target::<D, CasterBuilder::<usize>, _>(lhs, rhs),
        }
    }
}

impl Neg for Num {
    type Output = Result<Num, Error>;
    fn neg(self) -> Self::Output {Self::apply_monadic::<NegOp>(self)}
}

impl Num {
    pub fn cos(self) -> Result<Self, Error> {Self::apply_monadic::<CosOp>(self)}
}

impl Add for Num {
    type Output = Result<Num, Error>;
    fn add(self, rhs: Self) -> Self::Output {Self::apply_dyadic::<AddOp>(self, rhs)}
}

impl Sub for Num {
    type Output = Result<Num, Error>;
    fn sub(self, rhs: Self) -> Self::Output {Self::apply_dyadic::<SubOp>(self, rhs)}
}

impl Mul for Num {
    type Output = Result<Num, Error>;
    fn mul(self, rhs: Self) -> Self::Output {Self::apply_dyadic::<MulOp>(self, rhs)}
}

impl Div for Num {
    type Output = Result<Num, Error>;
    fn div(self, rhs: Self) -> Self::Output {Self::apply_dyadic::<DivOp>(self, rhs)}
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Int(i)   => write!(f, "{i}"),
            Num::Float(x) => write!(f, "{x}"),
            Num::USize(i) => write!(f, "{i}"),
        }
    }
}
