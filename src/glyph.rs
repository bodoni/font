/// A glyph.
pub struct Glyph {
    pub program: Vec<Operation>,
}

/// An operation.
pub enum Operation {
    /// Draw a line.
    LineTo(f32, f32),
    /// Move the cursor.
    MoveTo(f32, f32),
}

pub struct Builder {
    x: f32,
    y: f32,
    program: Vec<Operation>,
}

impl Builder {
    #[inline]
    pub fn new() -> Builder {
        Builder { x: 0.0, y: 0.0, program: vec![] }
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.program.push(Operation::LineTo(self.x, self.y));
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.program.push(Operation::MoveTo(self.x, self.y));
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(builder: Builder) -> Glyph {
        Glyph { program: builder.program }
    }
}
