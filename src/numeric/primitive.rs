use std::fmt::{Display, Debug};
use std::ops;

use num_traits;
use num_traits::cast::AsPrimitive;
use num_traits::ops::checked;
use num_traits::cast::NumCast;

macro_rules! add_checked_dyadic_trait {
    ($trait:ident, $pretrait:ident, $func:ident, $($prim:ty),+) => {
        pub trait $trait: Sized + ops::$pretrait<Self, Output=Self> {
            fn $func(&self, rhs: &Self) -> Option<Self>;
        }
        
        $(
            impl $trait for $prim {
                fn $func(&self, rhs: &Self) -> Option<Self> {
                    checked::$trait::$func(self, rhs)
                }
            }
        )+
    };
}

add_checked_dyadic_trait!(CheckedAdd, Add, checked_add, usize, i64, i128);
add_checked_dyadic_trait!(CheckedSub, Sub, checked_sub, usize, i64, i128);
add_checked_dyadic_trait!(CheckedMul, Mul, checked_mul, usize, i64, i128);
add_checked_dyadic_trait!(CheckedDiv, Div, checked_div, usize, i64, i128);

impl CheckedAdd for f64 {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        let r = self + rhs;
        if r.is_finite() {Some(r)} else {None}
    }
}


impl CheckedSub for f64 {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let r = self - rhs;
        if r.is_finite() {Some(r)} else {None}
    }
}

impl CheckedMul for f64 {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        let r = self * rhs;
        if r.is_finite() {Some(r)} else {None}
    }
}

impl CheckedDiv for f64 {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        let r = self / rhs;
        if r.is_finite() {Some(r)} else {None}
    }
}

macro_rules! add_checked_monadic_trait {
    ($trait:ident, $func:ident, $pretrait:ident, $prefunc:ident, $($prim:ty),+) => {
        pub trait $trait: Sized {
            fn $func(&self) -> Option<Self>;
        }
        
        $(
            impl $trait for $prim {
                fn $func(&self) -> Option<Self> {
                    Some(ops::$pretrait::$prefunc(self))
                }
            }
        )+
    };
}

add_checked_monadic_trait!(CheckedNeg, checked_neg, Neg, neg, i64, f64, i128); 

impl CheckedNeg for usize {
    fn checked_neg(&self) -> Option<Self> {None}
}

pub trait NumericPrimitive:
    Display + Debug + Copy +
    num_traits::Num + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedNeg +
    AsPrimitive<f64> + AsPrimitive<i64> + AsPrimitive<usize> + AsPrimitive<i128> +
    PartialOrd + NumCast
{}

impl NumericPrimitive for i64 {}
impl NumericPrimitive for usize {}
impl NumericPrimitive for f64 {}
impl NumericPrimitive for i128 {}
