use crate::error::*;
use super::*;

pub trait MonadicOp {
    fn func<T>(scalar: T) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat;
    
    fn target_scalar<T>(scalar: cardinality::Scalar<T>) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat
    {
        if let Value(value) = scalar {
            Self::func(value)
        } else {Ok(NaN)}
    }

    fn target_array<T>(mut array: cardinality::Array<T>) ->
        Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat
    {
        for val in array.iter_mut() {
            *val = Self::target_scalar(*val)?
        };
        Ok(array)
    }

    fn apply<T>(val: Number<T>) ->
        Result<Number<T>, Error> where
        T: NumericPrimitive + CastFromFloat
    {
        match val {
            Array(array)   => Ok(Self::target_array (array)?.into()),
            Scalar(scalar) => Ok(Self::target_scalar(scalar)?.into()),
        }
    }
}

pub trait FloatMonadicOp {
    fn float_func(a: f64) -> Result<Option<f64>, Error>;
}

impl<S> MonadicOp for S where
    S: FloatMonadicOp
{
    fn func<T>(val: T) ->
        Result<NumericValue<T>, Error> where
        T: NumericPrimitive + CastFromFloat
    {
        let Some(value) = Self::float_func(val.as_())? else {
            return Ok(NaN)
        };
        let Some(value) = CastFromFloat::cast(value) else {
            return Ok(NaN)
        };
        Ok(Value(value))
    }
}

pub trait DyadicOp {
    fn func<T>(lhs: T, rhs: T) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat;
    
    fn operator<T, U, C>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>, caster: C) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        if let (Value(lhs), Value(rhs)) = (lhs, rhs) {
            let (lhs, rhs) = caster.cast(lhs, rhs);
            let mid = Self::func(lhs, rhs)?;
            if let Value(mid) = mid {Ok(Value(caster.back_cast(mid)))}
            else {Ok(NaN)}
        } else {Ok(NaN)}
    }

    fn scalar_scalar<T, U, C>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>, caster: C) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        Self::operator(lhs, rhs, caster)
    }

    fn array_array<T, U, C>(mut lhs: cardinality::Array<T>, rhs: cardinality::Array<U>, caster: C) ->
        Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
   {
        if lhs.len() != rhs.len() {return Err(Error::LengthMismatch)};
        for (lhs, rhs) in lhs.iter_mut().zip(&rhs) {
            *lhs = Self::operator(*lhs, *rhs, caster)?
        };
        Ok(lhs)
    }

    fn array_scalar<T, U, C>(mut lhs: cardinality::Array<T>, rhs: cardinality::Scalar<U>, caster: C)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        for lhs in &mut lhs {
            *lhs = Self::operator(*lhs, rhs, caster)?
        };
        Ok(lhs)
    }

    fn scalar_array<T, U, C>(mut lhs: cardinality::Scalar<T>, rhs: cardinality::Array<U>, caster: C)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        for rhs in rhs {
            lhs = Self::operator(lhs, rhs, caster)?
        };
        Ok(lhs)
    }
    
    fn target_array<T, U, C>(lhs: cardinality::Array<T>, rhs: Number<U>, caster: C)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match rhs {
            Array(array)   => Self::array_array(lhs, array, caster),
            Scalar(scalar) => Self::array_scalar(lhs, scalar, caster),
        }
    }

    fn target_scalar<T, U, C>(lhs: cardinality::Scalar<T>, rhs: Number<U>, caster: C)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match rhs {
            Array(array)   => Self::scalar_array (lhs, array, caster),
            Scalar(scalar) => Self::scalar_scalar(lhs, scalar, caster),
        }
    }

    fn apply<T, U, C>(lhs: Number<T>, rhs: Number<U>, caster: C)
        -> Result<Number<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match lhs {
            Array(array)   => Ok(Self::target_array (array,  rhs, caster)?.into()),
            Scalar(scalar) => Ok(Self::target_scalar(scalar, rhs, caster)?.into()),
        }
    }
}

pub trait FloatDyadicOp {
    fn float_func(lhs: f64, rhs: f64) -> Result<Option<f64>, Error>;
}

impl<S> DyadicOp for S where
    S: FloatDyadicOp
{
    fn func<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat
    {
        let Some(value) = Self::float_func(lhs.as_(), rhs.as_())? else {
            return Ok(NaN)
        };
        let Some(value) = CastFromFloat::cast(value) else {
            return Ok(NaN)
        };
        Ok(Value(value))
    }
}
