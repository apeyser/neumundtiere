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

    fn apply_dyadic_target<D, T, C>(lhs: Number<T>, rhs: Num, caster: C) ->
        Result<Num, Error> where
        T: NumericPrimitive + CastFromFloat,
        D: DyadicOp,
        Num: From<Number<T>>,
        C: CasterBuilderTrait<T>,
        Caster::<T, i64>: CasterTrait<T, i64>,
        Caster::<T, f64>: CasterTrait<T, f64>,
        Caster::<T, usize>: CasterTrait<T, usize>,
    {
        match rhs {
            Num::Int(rhs)   => Ok(D::apply(lhs, rhs, caster.caster_i64())?.into()),
            Num::Float(rhs) => Ok(D::apply(lhs, rhs, caster.caster_f64())?.into()),
            Num::USize(rhs) => Ok(D::apply(lhs, rhs, caster.caster_usize())?.into()),
        }
    }

    pub fn apply_dyadic<D: DyadicOp>(self, rhs: Num) -> Result<Num, Error> {
        match self {
            Num::Int(lhs)   => Self::apply_dyadic_target::<D, _, _>(lhs, rhs, CasterBuilder::<i64>::new()),
            Num::Float(lhs) => Self::apply_dyadic_target::<D, _, _>(lhs, rhs, CasterBuilder::<f64>::new()),
            Num::USize(lhs) => Self::apply_dyadic_target::<D, _, _>(lhs, rhs, CasterBuilder::<usize>::new()),
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
