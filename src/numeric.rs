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

#[derive(Clone, Copy)]
pub struct CasterBuilder<T: NumericPrimitive + CastFromFloat> {
    lhs: PhantomData<T>,
}

impl<T> CasterBuilder<T> where
    T: NumericPrimitive + CastFromFloat {
    pub fn new() -> Self {Self {lhs: PhantomData}}
}

macro_rules! caster_builder_trait {
    ($($rhs:ident),+) => {
        paste! {
            pub trait CasterBuilderTrait<T: NumericPrimitive + CastFromFloat>: Copy+Clone {
                $(
                    fn [<caster_ $rhs>](&self) -> Caster<T, $rhs> {
                        Caster::<T, $rhs>::new()
                    }
                )+
            }
        }
    };
}

macro_rules! caster_builder {
    ($lhs:ty => ($($rhs:ident),+)) => {
        paste! {
            impl CasterBuilderTrait<$lhs> for CasterBuilder<$lhs> {}
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

impl<T, U> Caster<T, U> where
    T: NumericPrimitive + CastFromFloat,
    U: NumericPrimitive {
    pub fn new() -> Self {Self {lhs: PhantomData, rhs: PhantomData}}
}

pub trait CasterTrait<T, U>: Copy+Clone where
    T: NumericPrimitive + CastFromFloat,
    U: NumericPrimitive {
    type Mid: NumericPrimitive + CastFromFloat;
    fn cast(&self, lhs: T, rhs: U) -> (Self::Mid, Self::Mid);
    fn back_cast(&self, mid: Self::Mid) -> T;
}

macro_rules! caster {
    ($(($lhs:ty, $rhs:ty) => $mid:ty),+) => {
        $(
            impl CasterTrait<$lhs, $rhs> for Caster<$lhs, $rhs> {
                type Mid = $mid;
                fn cast(&self, lhs: $lhs, rhs: $rhs) -> (Self::Mid, Self::Mid) {(lhs as Self::Mid, rhs as Self::Mid)}
                fn back_cast(&self, mid: Self::Mid) -> $lhs {mid as $lhs}
            }
        )+
    };
}

caster!((i64, i64) => i64, (i64, f64) => f64, (i64, usize) => i128,
          (usize, i64) => i128, (usize, f64) => f64, (usize, usize) => usize,
          (f64, i64) => f64, (f64, f64) => f64, (f64, usize) => f64);

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
