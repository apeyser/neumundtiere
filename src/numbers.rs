// ;; -rust-

pub enum Error {
    DivideByZero,
    Overflow,
    Underflow,
}

type Array<T: std::Num> = Vec<T>;
type Scalar<T: std::Num> = T;

pub enum Num<T> {
    Array(Array<T>),
    Scalar(Scalar<T>),
}

pub enum Numbers {
    NaN,
    F64(Num<f64>),
    I64(Num<i32>),
}

struct NumOp {}

trait DyadicOp {
    fn operator<T> operator(lhs: Scalar<T>, rhs: Scalar<T>) -> Result<Scalar<T>, Error>;

    fn scalar_scalar<T, U>(lhs: Scalar<T>, rhs: Scalar<U>) ->  Result<Scalar<T>, Error> {
        Self::operator(lhs, rhs.into::<T>())
    }

    fn array_array<T, U>(lhs: mut Array<T>, rhs: Array<U>) -> Result<Array<T>, Error> {
        if lhs.len() != rhs.len() {return Err(NumOpError::LengthMismatch)};
        for (mut lhs, rhs) in lhs.iter().zip(&rhs) {
            a = Self::operator(lhs, rhs.into::<T>())?
        }
        Ok(lhs)
    }

    fn array_scalar<T, U>(lhs: mut Array<T>, rhs: Scalar<U>) -> Result<Array<T>, Error> {
        let rhs = rhs.into::<T>();
        for v in lhs {
            v = Self::operator(v, rhs)?
        };
        Ok(lhs)
    }

    fn scalar_array<T, U>(lhs: Scalar<T>, rhs: Array<U>) -> Result<Scalar<T>, Error> {
        let mut lhs = lhs;
        for v in rhs {
            lhs = Self::operator(lhs, v.into::<T>())?
        };
        Ok(lhs)
    }
    
    fn target_array<T, U>(array: Array<T>, rhs: Num<U>) -> Result<Array<T>, Error> {
        match rhs {
            Num<U>::Array(array)   => Self::array_array(lhs, rsh),
            Num<U>::Scalar(scalar) => Self::array_scalar(lhs, rhs),
        }
    }

    fn target_scalar<T, U>(scalar: Scalar<T>, rhs: Num<U>) -> Result<Scalar<T>, Error> {
        match rhs {
            Num<U>::Array(array)   => Self::scalar_array(lhs, rsh),
            Num<U>::Scaler(scalar) => Self::scalar_scalar(lhs, rhs),
        }
    }

    pub fn apply<T, U>(lhs: Num<T>, rhs: Num<U>) -> Result<Num<T>, Error> {
        match lhs {
            Num<T>::Array(array)   => Ok(Self::target_array(lhs, rhs)?.into()),
            Num<T>::Scalar(scalar) => Ok(Self::target_scalar(lhs, rhs)?.into()),
        }
    }
}

trait Num {}

trait NumOps {
    
}

struct<T> SingleNum<T>
where T: NumOps {
    
}

trait Num
{
    type Container = SingleNum<T>;
    
}

struct Add;
struct Sub;
struct Mul;
struct Div;

impl DyadicOp for Add {
    pub fn operator<T: Num::CheckedAdd> (lhs: Scalar<T>, rhs: Scalar<T>) -> Result<Scalar<T>, Error> {
        let Some(r) = lhs.checked_add(rhs) else {return Err(Error::Overflow)};
        Ok(r)
    }
}

impl DyadicOp for Sub {
    pub fn operator<T: Num::CheckedSub> (lhs: Scalar<T>, rhs: Scalar<T>) -> Result<Scalar<T>, Error> {
        let Some(r) = lhs.checked_sub(rhs) else {return Err(Error::Underflow)};
        Ok(r)
    }
}

impl DyadicOp for Mul {
    pub fn operator<T> (lhs: Scalar<T>, rhs: Scalar<T>) -> Result<Scalar<T>, Error> {
        Ok(lhs * rhs)
    }
}

impl DyadicOp for Div {
    pub fn operator<T> (lhs: Scalar<T>, rhs: Scalar<T>) -> Result<Scalar<T>, Error> {
        if rhs == 0 {
            Err(Error::DivideByZero)
        } else {
            Ok(lhs / rhs)
        }
    }
}



    
