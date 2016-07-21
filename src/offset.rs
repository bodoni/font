use std::ops::{Add, AddAssign, Div, Neg, Sub};

/// An offset.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Offset(pub f32, pub f32);

impl Offset {
    /// Create an offset.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Offset(x, y)
    }

    /// Create a zero offset.
    #[inline]
    pub fn zero() -> Self {
        Offset(0.0, 0.0)
    }
}

impl From<Offset> for (f32, f32) {
    #[inline]
    fn from(offset: Offset) -> Self {
        (offset.0, offset.1)
    }
}

impl From<(f32, f32)> for Offset {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Offset(x, y)
    }
}

impl Add for Offset {
    type Output = Offset;

    #[inline]
    fn add(mut self, other: Offset) -> Offset {
        self.0 += other.0;
        self.1 += other.1;
        self
    }
}

impl AddAssign for Offset {
    #[inline]
    fn add_assign(&mut self, other: Offset) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Div<f32> for Offset {
    type Output = Offset;

    #[inline]
    fn div(mut self, other: f32) -> Offset {
        self.0 /= other;
        self.1 /= other;
        self
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

impl Sub for Offset {
    type Output = Offset;

    #[inline]
    fn sub(self, other: Offset) -> Offset {
        self + (-other)
    }
}
