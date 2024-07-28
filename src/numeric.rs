use std::fmt::{self, Debug};
use std::marker::PhantomData;
use itertools::Itertools;
use num_traits::cast::cast;
use paste::paste;

pub mod ops;
pub mod ops_defs;
pub mod primitive;

use primitive::NumericPrimitive;

pub trait CastFromFloat: NumericPrimitive {
    fn cast(f: f64) -> Option<Self>;
}

impl CastFromFloat for f64 {
    fn cast(f: f64) -> Option<Self> {Some(f)}
}

macro_rules! cast_from_float {
    ($($prim:ident),+) => {
        $(
            impl CastFromFloat for $prim {
                fn cast(f: f64) -> Option<Self> {
                    if ! f.is_finite() ||
                        f > $prim::MAX as f64 ||
                        f < $prim::MIN as f64
                    {None} else {
                        Some(f.round() as $prim)
                    }
                }
            }
        )+
    };
}

cast_from_float!(i64, usize, i128);

pub trait CastFrom128: NumericPrimitive {
    fn cast(f: i128) -> Option<Self>;
}

macro_rules! cast_from_128 {
    ($($prim:ident),+) => {
        $(
            impl CastFrom128 for $prim {
                fn cast(i: i128) -> Option<Self> {
                    if i > $prim::MAX as i128 ||
                        i < $prim::MIN as i128
                    {None} else {Some(i as $prim)}
                }
            }
        )+
    };
}

cast_from_128!(i64, usize);

pub struct CasterBuilder<T: NumericPrimitive + CastFromFloat> {
    lhs: PhantomData<T>,
}

macro_rules! caster_builder_trait {
    ($($rhs:ident),+) => {
        pub trait CasterBuilderTrait<T: NumericPrimitive + CastFromFloat> {
            paste! {
                $(
                    #[allow(non_camel_case_types)]
                    type [<Caster_ $rhs>]: CasterTrait<T, $rhs>;
                )+
            }
        }
    };
}

macro_rules! caster_builder {
    ($lhs:ty => ($($rhs:ident),+)) => {
        impl CasterBuilderTrait<$lhs> for CasterBuilder<$lhs> {
            paste! {
                $(
                    #[allow(non_camel_case_types)]
                    type [<Caster_ $rhs>] = Caster<$lhs, $rhs>;
                )+
            }
        }
    };
}

caster_builder_trait!(i64, f64, usize);
caster_builder!(i64 => (i64, usize, f64));
caster_builder!(usize => (i64, usize, f64));
caster_builder!(f64 => (i64, usize, f64));

#[derive(Clone, Copy)]
pub struct Caster<T, U> where
    T: NumericPrimitive + CastFromFloat,
    U: NumericPrimitive {
    lhs: PhantomData<T>,
    rhs: PhantomData<U>,
}

pub trait CasterTrait<T, U>: Copy+Clone where
    T: NumericPrimitive + CastFromFloat,
    U: NumericPrimitive {
    type Mid: NumericPrimitive + CastFromFloat;
    fn cast(lhs: T, rhs: U) -> (Self::Mid, Self::Mid);
    fn back_cast(mid: Self::Mid) -> Option<T>;
}

macro_rules! caster_base {
    ($(($lhs:ty, $rhs:ty, $mid:ident, $cast:expr, $back_cast:expr)),+) => {
        $(
            impl CasterTrait<$lhs, $rhs> for Caster<$lhs, $rhs> {
                type Mid = $mid;
                fn cast(lhs: $lhs, rhs: $rhs) -> (Self::Mid, Self::Mid) {
                    $cast(lhs, rhs)
                }
                fn back_cast(mid: Self::Mid) -> Option<$lhs> {
                    $back_cast(mid)
                }
            }
        )+
    };
}

macro_rules! caster_simple {
    ($(($lhs:ty, $rhs:ty) => $mid:ident),+) => {
        caster_base!($((
            $lhs, $rhs, $mid,
            |lhs, rhs| {(lhs as Self::Mid, rhs as Self::Mid)},
            |mid| {Some(mid as $lhs)}
        )),+);
    };
}

macro_rules! caster_back {
    ($(($lhs:ty, $rhs:ty) => $mid:ident $back_cast:expr),+) => {
        caster_base!($((
            $lhs, $rhs, $mid,
            |lhs, rhs| {(lhs as Self::Mid, rhs as Self::Mid)},
            $back_cast
        )),+);
    };
}

caster_simple!(
    (i64,   i64)   => i64,
    (usize, usize) => usize,
    (f64,   f64)   => f64,
    (f64,   i64)   => f64,
    (f64,   usize) => f64
);

caster_back!(
    (i64,   f64)   => f64  CastFromFloat::cast,
    (usize, f64)   => f64  CastFromFloat::cast,
    (i64,   usize) => i128 CastFrom128::cast,
    (usize, i64)   => i128 CastFrom128::cast
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericValue<T: NumericPrimitive> {
    Value(T),
    NaN,
}

pub use NumericValue::{Value, NaN};

impl<T: NumericPrimitive> NumericValue<T> {
    pub fn is_nan(self) -> bool {self == Self::NaN}
    pub fn to_primitive(self) -> T {
        match self {
            Value(value) => value,
            _ => panic!("NaN in to_primitive"),
        }
    }
    pub fn from_primitive(value: T) -> Self {Value(value)}
    pub fn to_value<U: NumericPrimitive>(self) -> NumericValue<U> {
        if let Value(value) = self {
            if let Some(value) = cast::<T, U>(value) {
                Value(value)
            } else {NaN}
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
    pub type Scalar<T> = super::NumericValue<T>; //where T: super::NumericPrimitive;
    pub type Array<T> = Vec<Scalar<T>>; //where T: super::NumericPrimitive;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number<T: NumericPrimitive> {
    Array(cardinality::Array<T>),
    Scalar(cardinality::Scalar<T>),
}

pub use Number::{Array, Scalar};

impl<T> From<cardinality::Scalar<T>> for Number<T> where
    T: NumericPrimitive {
    fn from(item: cardinality::Scalar<T>) -> Number<T> {Scalar(item)}
}

impl<T> From<cardinality::Array<T>> for Number<T> where
    T: NumericPrimitive {
    fn from(item: cardinality::Array<T>) -> Number<T> {Array(item)}
}

impl<T: NumericPrimitive> fmt::Display for Number<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            Array(array) => {
                let s = array.iter().map(|scalar| format!("{scalar}")).join(" ");
                f.write_str(s.as_str())
            },
        }
    }
}
