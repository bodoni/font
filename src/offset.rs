use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Number;

/// An offset.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Offset(pub Number, pub Number);

impl From<Offset> for (Number, Number) {
    #[inline]
    fn from(offset: Offset) -> Self {
        (offset.0, offset.1)
    }
}

macro_rules! implement(
    (($x:ty, $y:ty)) => (
        impl From<($x, $y)> for Offset {
            #[inline]
            fn from((x, y): ($x, $y)) -> Self {
                Offset(Number::from(x), Number::from(y))
            }
        }
    );
    ($z:ty) => (
        impl From<$z> for Offset {
            #[inline]
            fn from(z: $z) -> Self {
                Offset(Number::from(z), Number::from(z))
            }
        }

        impl Div<$z> for Offset {
            type Output = Offset;

            #[inline]
            fn div(mut self, other: $z) -> Offset {
                self /= other;
                self
            }
        }

        impl DivAssign<$z> for Offset {
            #[inline]
            fn div_assign(&mut self, other: $z) {
                self.0 /= Number::from(other);
                self.1 /= Number::from(other);
            }
        }

        impl Mul<$z> for Offset {
            type Output = Offset;

            #[inline]
            fn mul(mut self, other: $z) -> Offset {
                self *= other;
                self
            }
        }

        impl MulAssign<$z> for Offset {
            #[inline]
            fn mul_assign(&mut self, other: $z) {
                self.0 *= Number::from(other);
                self.1 *= Number::from(other);
            }
        }
    );
);

implement!(f32);
implement!(i16);
implement!((f32, f32));
implement!((i16, i16));

impl<T> Add<T> for Offset
where
    T: Into<Offset>,
{
    type Output = Offset;

    #[inline]
    fn add(mut self, other: T) -> Offset {
        self += other;
        self
    }
}

impl<T> AddAssign<T> for Offset
where
    T: Into<Offset>,
{
    #[inline]
    fn add_assign(&mut self, other: T) {
        let Offset(x, y) = other.into();
        self.0 += x;
        self.1 += y;
    }
}

impl Neg for Offset {
    type Output = Offset;

    #[inline]
    fn neg(mut self) -> Offset {
        self.0 = -self.0;
        self.1 = -self.1;
        self
    }
}

impl<T> Sub<T> for Offset
where
    T: Into<Offset>,
{
    type Output = Offset;

    #[inline]
    fn sub(mut self, other: T) -> Offset {
        self -= other;
        self
    }
}

impl<T> SubAssign<T> for Offset
where
    T: Into<Offset>,
{
    #[inline]
    fn sub_assign(&mut self, other: T) {
        let Offset(x, y) = other.into();
        self.0 -= x;
        self.1 -= y;
    }
}
