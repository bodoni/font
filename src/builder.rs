use std::mem;

use crate::glyph::{Contour, Glyph, Segment};
use crate::{Number, Offset};

#[derive(Default)]
pub struct Builder {
    contour: Contour,
    glyph: Glyph,
    offset: Offset,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Builder {
    pub fn move_absolute<T: Into<Offset>>(&mut self, value: T) {
        let last_position = match self.glyph.len() {
            0 => Offset::default(),
            count => self.glyph[count - 1].position,
        };
        let value = self.offset + value;
        self.contour.offset = value - last_position;
        self.contour.position = value;
    }

    pub fn move_relative<T: Into<Offset>>(&mut self, value: T) {
        let value = value.into();
        self.contour.offset += value;
        self.contour.position += value;
    }

    pub fn move_control<T: Into<Offset>>(&mut self, a: T) {
        let b = match self.contour.segments.get_mut(0) {
            Some(&mut Segment::Quadratic(ref mut b, _)) => b,
            Some(&mut Segment::Cubic(ref mut b, _, _)) => b,
            _ => unreachable!(),
        };
        *b = a.into();
    }
}

impl Builder {
    pub fn flush(&mut self) {
        if self.contour.is_empty() {
            return;
        }
        self.glyph
            .contours
            .push(mem::replace(&mut self.contour, Default::default()));
    }

    pub fn nest<T, U, F>(&mut self, offset: T, build: F) -> U
    where
        T: Into<Offset>,
        F: Fn(&mut Builder) -> U,
    {
        let offset = offset.into();
        self.offset += offset;
        let result = build(self);
        self.offset -= offset;
        result
    }
}

impl Builder {
    pub fn add_linear<T: Into<Offset>>(&mut self, a: T) {
        self.contour.segments.push(Segment::Linear(a.into()));
    }

    pub fn add_quadratic<T: Into<Offset>>(&mut self, a: T, b: T) {
        self.contour
            .segments
            .push(Segment::Quadratic(a.into(), b.into()));
    }

    pub fn add_cubic<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        self.contour
            .segments
            .push(Segment::Cubic(a.into(), b.into(), c.into()));
    }
}

impl Builder {
    #[inline]
    pub fn set_bounding_box<T: Into<Number>>(&mut self, min_x: T, min_y: T, max_x: T, max_y: T) {
        self.glyph.bounding_box = (min_x.into(), min_y.into(), max_x.into(), max_y.into());
    }

    #[inline]
    pub fn set_horizontal_metrics(&mut self, metrics: (Number, Number)) {
        self.glyph.advance_width = metrics.0;
        self.glyph.side_bearings.0 = metrics.1;
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
