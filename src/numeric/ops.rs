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
    
    fn operator<T, U, C>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        if let (Value(lhs), Value(rhs)) = (lhs, rhs) {
            let (lhs, rhs) = C::cast(lhs, rhs);
            let mid = Self::func(lhs, rhs)?;
            if let Value(mid) = mid {
                if let Some(mid) = C::back_cast(mid) {
                    Ok(Value(mid))
                } else {Ok(NaN)}
            } else {Ok(NaN)}
        } else {Ok(NaN)}
    }

    fn scalar_scalar<T, U, C>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>) ->
        Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        Self::operator::<_, _, C>(lhs, rhs)
    }

    fn array_array<T, U, C>(mut lhs: cardinality::Array<T>, rhs: cardinality::Array<U>) ->
        Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
   {
        if lhs.len() != rhs.len() {return Err(Error::LengthMismatch)};
        for (lhs, rhs) in lhs.iter_mut().zip(&rhs) {
            *lhs = Self::operator::<_, _, C>(*lhs, *rhs)?
        };
        Ok(lhs)
    }

    fn array_scalar<T, U, C>(mut lhs: cardinality::Array<T>, rhs: cardinality::Scalar<U>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        for lhs in &mut lhs {
            *lhs = Self::operator::<_, _, C>(*lhs, rhs)?
        };
        Ok(lhs)
    }

    fn scalar_array<T, U, C>(mut lhs: cardinality::Scalar<T>, rhs: cardinality::Array<U>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        for rhs in rhs {
            lhs = Self::operator::<_, _, C>(lhs, rhs)?
        };
        Ok(lhs)
    }
    
    fn target_array<T, U, C>(lhs: cardinality::Array<T>, rhs: Number<U>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match rhs {
            Array(array)   => Self::array_array::<_, _, C>(lhs, array),
            Scalar(scalar) => Self::array_scalar::<_, _, C>(lhs, scalar),
        }
    }

    fn target_scalar<T, U, C>(lhs: cardinality::Scalar<T>, rhs: Number<U>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match rhs {
            Array(array)   => Self::scalar_array::<_, _, C>(lhs, array),
            Scalar(scalar) => Self::scalar_scalar::<_, _, C>(lhs, scalar),
        }
    }

    fn apply<T, U, C>(lhs: Number<T>, rhs: Number<U>)
        -> Result<Number<T>, Error> where
        T: NumericPrimitive + CastFromFloat,
        U: NumericPrimitive,
        C: CasterTrait<T, U>
    {
        match lhs {
            Array(array)   => Ok(Self::target_array::<_, _, C>(array,  rhs)?.into()),
            Scalar(scalar) => Ok(Self::target_scalar::<_, _, C>(scalar, rhs)?.into()),
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
