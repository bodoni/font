use crate::glyph::{Contour, Glyph, Segment};
use crate::{Number, Offset};

pub struct Builder {
    contour: Contour,
    glyph: Glyph,

    offset: Offset,
    scale: Number,
}

impl Builder {
    pub fn flush(&mut self) {
        use std::mem;

        if self.contour.is_empty() {
            return;
        }
        self.glyph
            .contours
            .push(mem::replace(&mut self.contour, Default::default()));
    }

    pub fn nest<T, U, F>(&mut self, offset: T, scale: Number, build: F) -> U
    where
        T: Into<Offset>,
        F: Fn(&mut Builder) -> U,
    {
        let offset = offset.into();
        self.offset += offset;
        self.scale *= scale;
        let result = build(self);
        self.scale /= scale;
        self.offset -= offset;
        result
    }

    #[inline]
    pub fn transform<T: Into<Offset>>(&self, value: T) -> Offset {
        value.into() * self.scale
    }
}

impl Builder {
    pub fn move_absolute<T: Into<Offset>>(&mut self, a: T) {
        let last_position = match self.glyph.len() {
            0 => Offset::default(),
            count => self.glyph[count - 1].position,
        };
        let a = self.offset + self.transform(a);
        self.contour.offset = a - last_position;
        self.contour.position = a;
    }

    pub fn move_relative<T: Into<Offset>>(&mut self, a: T) {
        let a = self.transform(a);
        self.contour.offset += a;
        self.contour.position += a;
    }

    pub fn move_control<T: Into<Offset>>(&mut self, a: T) {
        let a = self.transform(a);
        let b = match self.contour.segments.get_mut(0) {
            Some(&mut Segment::Quadratic(ref mut b, _)) => b,
            Some(&mut Segment::Cubic(ref mut b, _, _)) => b,
            _ => unreachable!(),
        };
        *b = a;
    }
}

impl Builder {
    pub fn add_linear<T: Into<Offset>>(&mut self, a: T) {
        self.contour
            .segments
            .push(Segment::Linear(self.transform(a)));
    }

    pub fn add_quadratic<T: Into<Offset>>(&mut self, a: T, b: T) {
        self.contour
            .segments
            .push(Segment::Quadratic(self.transform(a), self.transform(b)));
    }

    pub fn add_cubic<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        self.contour.segments.push(Segment::Cubic(
            self.transform(a),
            self.transform(b),
            self.transform(c),
        ));
    }
}

impl Builder {
    #[inline]
    pub fn set_bounding_box<T: Into<Number>>(
        &mut self,
        (min_x, min_y, max_x, max_y): (T, T, T, T),
    ) {
        self.glyph.bounding_box = (min_x.into(), min_y.into(), max_x.into(), max_y.into());
    }

    #[inline]
    pub fn set_horizontal_metrics(&mut self, (advance_width, left_side_bearing): (Number, Number)) {
        self.glyph.advance_width = advance_width;
        self.glyph.side_bearings.0 = left_side_bearing;
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self {
            contour: Default::default(),
            glyph: Default::default(),

            offset: Default::default(),
            scale: 1.0,
        }
    }
}

impl From<Builder> for Glyph {
    fn from(builder: Builder) -> Glyph {
        let Builder { mut glyph, .. } = builder;
        let width = glyph.bounding_box.2 - glyph.bounding_box.0;
        glyph.side_bearings.1 = glyph.advance_width - (glyph.side_bearings.0 + width);
        glyph
    }
}
