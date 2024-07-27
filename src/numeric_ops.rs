pub use super::numeric::*;

trait MonadicOp {
    fn op<T>(scalar: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive;
    
    fn operator<T>(scalar: cardinality::Scalar<T>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive
    {
        if let Value(value) = scalar {Self::op(value)} else {Ok(NaN)}
    }

    fn target_scalar<T>(scalar: cardinality::Scalar<T>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive
    {
        Self::operator(scalar)
    }

    fn target_array<T>(val: mut cardinality::Array<T>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive
    {
        for lhs in lhs.iter() {
            lhs = Self::operator(lhs)?
        };
        Ok(lhs)
    }

    pub fn apply<T>(val: Number<T>)
        -> Result<Number<T>, Error> where
        T: NumericPrimitive
    {
        match val {
            Array(array)   => Ok(Self::target_array (array)?.into()),
            Scalar(scalar) => Ok(Self::target_scalar(scalar)?.into()),
        }
    }
}

trait FloatMonadicOp: MonadicOp {
    fn floatop(a: f64) -> Result<Option<f64>, Error>;
    fn op<T>(val: T) -> Result<NumericValue<T>, Error> where
        T: NumericPrimitive
    {
        match Caster<f64, T>::cast(Self::floatop(a.as_()))? {
            None => Ok(NaN),
            Some(scalar) => Ok(Value(scalar)),
        }
    }
}

trait DyadicOp {
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive;
    
    fn operator<T, U>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        if let (Value(lhs), Value(rhs)) = (lhs, rhs) {
            let (lhs, rhs) = CommonDyadicCast<T, U>::cast(lhs, rhs);
            Ok(Self::op(lhs, rhs)?.into())
        } else {
            Ok(NaN)
        }
    }

    fn scalar_scalar<T, U>(lhs: cardinality::Scalar<T>, rhs: cardinality::Scalar<U>)
        ->  Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        Self::operator(lhs, rhs)
    }

    fn array_array<T, U>(lhs: mut cardinality::Array<T>, rhs: cardinality::Array<U>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        if lhs.len() != rhs.len() {return Err(Error::LengthMismatch)};
        for (mut lhs, rhs) in lhs.iter().zip(&rhs) {
            lhs = Self::operator(lhs, rhs)?
        };
        Ok(lhs)
    }

    fn array_scalar<T, U>(lhs: mut cardinality::Array<T>, rhs: cardinality::Scalar<U>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        let (_, rhs) = CommonDyadicCast<T, U>::cast(T::zero(), rhs);
        for lhs in lhs {
            lhs = Self::operator(lhs, rhs)?
        };
        Ok(lhs)
    }

    fn scalar_array<T, U>(lhs: mut cardinality::Scalar<T>, rhs cardinality::Array<U>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        for rhs in rhs {
            lhs = Self::operator(lhs, rhs)?
        };
        Ok(lhs)
    }
    
    fn target_array<T, U>(lhs: mut cardinality::Array<T>, rhs: Number<U>)
        -> Result<cardinality::Array<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        match rhs {
            Array(array)   => Self::array_array(lhs, array),
            Scalar(scalar) => Self::array_scalar(lhs, scalar),
        }
    }

    fn target_scalar<T, U>(lhs: cardinality::Scalar<T>, rhs: Number<U>)
        -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive,
        U: NumericPrimitive
    {
        match rhs {
            Array(array)   => Self::scalar_array (lhs, array),
            Scalar(scalar) => Self::scalar_scalar(lhs, scalar),
        }
    }

    pub fn apply<T, U>(lhs: Number<T>, rhs: Number<U>)
        -> Result<Number<T>, Error> where
        T: NumericPrimtive,
        U: NumericPrimtive
    {
        match lhs {
            Array(array)   => Ok(Self::target_array (array,  rhs)?.into()),
            Scalar(scalar) => Ok(Self::target_scalar(scalar, rhs)?.into()),
        }
    }
}

trait FloatDyadicOp: DyadicOp {
    fn floatop(lhs: f64, rhs: f64) -> Result<Option<f64>, Error>;
    fn op<T>(lhs: T, rhs: T) -> Result<cardinality::Scalar<T>, Error> where
        T: NumericPrimitive
    {
        match Caster<f64, T>::cast(Self::floatop(lhs.as_(), rhs.as_()))? {
            None => Ok(NaN),
            Some(value) => Ok(Value(value)),
        }
    }
}
