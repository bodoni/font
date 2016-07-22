use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// An offset.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Offset(pub f32, pub f32);

macro_rules! implement(
    (($x:ty, $y:ty)) => (
        impl From<($x, $y)> for Offset {
            #[inline]
            fn from((x, y): ($x, $y)) -> Self {
                Offset(x as f32, y as f32)
            }
        }

        impl From<Offset> for ($x, $y) {
            #[inline]
            fn from(offset: Offset) -> Self {
                (offset.0 as $x, offset.1 as $y)
            }
        }
    );
    ($z:ty) => (
        impl From<$z> for Offset {
            #[inline]
            fn from(z: $z) -> Self {
                Offset(z as f32, z as f32)
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
                self.0 /= other as f32;
                self.1 /= other as f32;
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
                self.0 *= other as f32;
                self.1 *= other as f32;
            }
        }
    );
);

implement!(f32);
implement!(i16);
implement!((f32, f32));
implement!((i16, i16));

impl<T> Add<T> for Offset where T: Into<Offset> {
    type Output = Offset;

    #[inline]
    fn add(mut self, other: T) -> Offset {
        self += other;
        self
    }
}

impl<T> AddAssign<T> for Offset where T: Into<Offset> {
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

impl<T> Sub<T> for Offset where T: Into<Offset> {
    type Output = Offset;

    #[inline]
    fn sub(mut self, other: T) -> Offset {
        self -= other;
        self
    }
}

impl<T> SubAssign<T> for Offset where T: Into<Offset> {
    #[inline]
    fn sub_assign(&mut self, other: T) {
        let Offset(x, y) = other.into();
        self.0 -= x;
        self.1 -= y;
    }
}
