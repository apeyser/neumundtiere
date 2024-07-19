use std::fmt::{self, Debug};
use std::ops::{Add,Sub,Div,Mul,Neg};

use num_traits::cast::AsPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Num {
    Int(i64),
    Float(f64),
    NaN,
}

impl From<i64> for Num {
    fn from(item: i64) -> Self {Num::Int(item)}
}

impl From<f64> for Num {
    fn from(item: f64) -> Self {Num::Float(item)}
}

trait Conv<T: AsPrimitive<f64>> {
    fn convert(r: f64) -> Self;
}

impl Conv<i64> for Num {
    fn convert(r: f64) -> Num {
         if ! r.is_finite() || r > i64::MAX as f64 || r < i64::MIN as f64  {
            Num::NaN
         } else {
             (r.round() as i64).into()
         }
    }
}

impl Conv<f64> for Num {
    fn convert(r: f64) -> Num {
        r.into()
    }
}

trait MonadicOp {
    fn op(a: f64) -> f64;

    fn exec<T>(a: T) -> Num
    where T: AsPrimitive<f64>,
          Num: Conv<T>,
    {
        <Num as Conv<T>>::convert(Self::op(a.as_()))
    }

    fn apply(a: Num) -> Num {
        match a {
            Num::Int(a)   => Self::exec(a),
            Num::Float(a) => Self::exec(a),
            Num::NaN => Num::NaN,
        }
    }
}

struct NegOp;
impl MonadicOp for NegOp {
    fn op(a: f64) -> f64 {-a}
}

impl Neg for Num {
    type Output = Num;
    fn neg(self) -> Self::Output {
        NegOp::apply(self)
    }
}

trait DyadicOp {
    fn op(a: f64, b: f64) -> f64;

    fn exec<T, U>(a: T, b: U) -> Num
    where T: AsPrimitive<f64>,
          U: AsPrimitive<f64>,
          Num: Conv<T>
    {
        <Num as Conv<T>>::convert(Self::op(a.as_(), b.as_()))
    }

    fn apply(a: Num, b: Num) -> Num {
        match (a, b) {
            (Num::Int(a),   Num::Int(b))   => Self::exec(a, b),
            (Num::Int(a),   Num::Float(b)) => Self::exec(a, b),
            (Num::Float(a), Num::Float(b)) => Self::exec(a, b),
            (Num::Float(a), Num::Int(b))   => Self::exec(a, b),
            (Num::NaN, _) => Num::NaN,
            (_, Num::NaN) => Num::NaN,
        }
    }
}

struct AddOp;
impl DyadicOp for AddOp {
    fn op(a: f64, b: f64) -> f64 {a+b}
}

impl Add for Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        AddOp::apply(self, rhs)
    }
}

struct SubOp;
impl DyadicOp for SubOp {
    fn op(a: f64, b: f64) -> f64 {a-b}
}

impl Sub for Num {
    type Output = Num;
    fn sub(self, rhs: Self) -> Self::Output {
        SubOp::apply(self, rhs)
    }
}

struct MulOp;
impl DyadicOp for MulOp {
    fn op(a: f64, b: f64) -> f64 {a*b}
}

impl Mul for Num {
    type Output = Num;
    fn mul(self, rhs: Self) -> Self::Output {
        MulOp::apply(self, rhs)
    }
}

struct DivOp;
impl DyadicOp for DivOp {
    fn op(a: f64, b: f64) -> f64 {a/b}
}

impl Div for Num {
    type Output = Num;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs {
            Num::Int(0) => Num::NaN,
            _ => DivOp::apply(self, rhs)
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Num::NaN      => write!(f, "NaN"),
            Num::Int(i)   => write!(f, "{i}"),
            Num::Float(x) => write!(f, "{x:+e}"),
        }
    }
}
