use std::ops::Deref;

use Point;

/// A glyph.
pub struct Glyph {
    /// The program.
    pub program: Vec<Operation>,
}

/// An operation.
pub enum Operation {
    /// Draw a cubic Bézier curve.
    CurveTo(Point, Point, Point),
    /// Draw a line.
    LineTo(Point),
    /// Move the cursor.
    MoveTo(Point),
}

pub struct Builder {
    cursor: Point,
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
    pub fn new() -> Builder {
        Builder { cursor: (0.0, 0.0), program: vec![] }
    }

    pub fn curve_to(&mut self, a: Offset, b: Offset, c: Offset) {
        let a = (self.cursor.0 + a.0, self.cursor.1 + a.1);
        let b = (a.0 + b.0, a.1 + b.1);
        let c = (b.0 + c.0, b.1 + c.1);
        self.cursor = c;
        self.program.push(Operation::CurveTo(a, b, c));
    }

    pub fn line_to(&mut self, a: Offset) {
        self.cursor = (self.cursor.0 + a.0, self.cursor.1 + a.1);
        self.program.push(Operation::LineTo(self.cursor));
    }

    pub fn move_to(&mut self, a: Offset) {
        self.cursor = (self.cursor.0 + a.0, self.cursor.1 + a.1);
        self.program.push(Operation::MoveTo(self.cursor));
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(builder: Builder) -> Glyph {
        Glyph { program: builder.program }
    }
}
