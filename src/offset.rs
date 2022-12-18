use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// An offset.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Offset(pub f32, pub f32);

impl Offset {
    // Check if the offset is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0.0 && self.1 == 0.0
    }
}

macro_rules! extremum(
    ($self:expr, $other:expr, $function:ident) => (
        if $self.is_nan() {
            $other
        } else if $other.is_nan() {
            $self
        } else {
            $self.$function($other)
        }
    )
);

impl Offset {
    /// Create an undefined offset.
    #[inline]
    pub fn undefined() -> Self {
        Self(f32::NAN, f32::NAN)
    }

    /// Return the coordinate-wise maximum ignoring undefined values.
    pub fn max(&self, other: Self) -> Self {
        let x = extremum!(self.0, other.0, max);
        let y = extremum!(self.1, other.1, max);
        Self(x, y)
    }

    /// Return the coordinate-wise minimum ignoring undefined values.
    pub fn min(&self, other: Self) -> Self {
        let x = extremum!(self.0, other.0, min);
        let y = extremum!(self.1, other.1, min);
        Self(x, y)
    }
}

impl From<Offset> for (f32, f32) {
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
                Offset(f32::from(x), f32::from(y))
            }
        }
    );
    ($z:ty) => (
        impl From<$z> for Offset {
            #[inline]
            fn from(z: $z) -> Self {
                Offset(f32::from(z), f32::from(z))
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
                self.0 /= f32::from(other);
                self.1 /= f32::from(other);
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
                self.0 *= f32::from(other);
                self.1 *= f32::from(other);
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
