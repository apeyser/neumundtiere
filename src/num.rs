use std::fmt;

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

impl From<usize> for Num {
    fn from(item: i64) -> Self {Num::USize(item.into())}
}

impl From<Vec<usize>> for Num {
    fn from(item: Vec<usize>) -> Self {Num::USize(item.into())}
}

impl From<f64> for Num {
    fn from(item: f64) -> Self {Num::Float(item.into())}
}

impl From<Vec<f64>> for Num {
    fn from(item: Vec<f64>) -> Self {Num::Float(item.into())}
}

impl Add for Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        AddOp::apply(self, rhs)
    }
}

impl Sub for Num {
    type Output = Num;
    fn sub(self, rhs: Self) -> Self::Output {
        SubOp::apply(self, rhs)
    }
}

impl Mul for Num {
    type Output = Num;
    fn mul(self, rhs: Self) -> Self::Output {
        MulOp::apply(self, rhs)
    }
}

impl Div for Num {
    type Output = Num;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs {
            Num::Int(0)|Num::Float(0.)|Num::USize(0) => Num::NaN,
            _ => DivOp::apply(self, rhs)
        }
    }
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
