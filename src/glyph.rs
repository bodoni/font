use std::ops::Deref;

use Point;

/// A glyph.
#[derive(Clone, Debug)]
pub struct Glyph {
    /// The program.
    pub program: Vec<Operation>,
}

/// An operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    /// Move the current point.
    Move(Point),
    /// Append a line to the current point.
    Line(Point),
    /// Append a Bézier curve to the current point.
    Curve(Curve),
}

/// A curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Curve {
    /// A quadratic Bézier curve.
    Quadratic(Point, Point),
    /// A cubic Bézier curve.
    Cubic(Point, Point, Point),
}

pub struct Builder {
    point: Point,
    program: Vec<Operation>,
}

pub type Offset = (f32, f32);

impl Deref for Glyph {
    type Target = [Operation];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.program
    }
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Builder { point: (0.0, 0.0), program: vec![] }
    }

    pub fn move_to(&mut self, a: Offset) {
        self.point = (self.point.0 + a.0, self.point.1 + a.1);
        self.program.push(Operation::Move(self.point));
    }

    pub fn line_to(&mut self, a: Offset) {
        self.point = (self.point.0 + a.0, self.point.1 + a.1);
        self.program.push(Operation::Line(self.point));
    }

    pub fn cubic_to(&mut self, a: Offset, b: Offset, c: Offset) {
        let a = (self.point.0 + a.0, self.point.1 + a.1);
        let b = (a.0 + b.0, a.1 + b.1);
        let c = (b.0 + c.0, b.1 + c.1);
        self.point = c;
        self.program.push(Operation::Curve(Curve::Cubic(a, b, c)));
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(builder: Builder) -> Glyph {
        Glyph { program: builder.program }
    }
}
