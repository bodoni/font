use std::mem;

use {Number, Offset};
use glyph::{Contour, Glyph, Segment};

#[derive(Default)]
pub struct Builder {
    start: Offset,
    position: Offset,
    compensation: Option<Offset>,
    contour: Contour,
    glyph: Glyph,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_cubic<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        let (a, b, c) = (self.compensate(a), b.into(), c.into());
        self.position += a;
        self.position += b;
        self.position += c;
        self.contour.segments.push(Segment::Cubic(a, b, c));
    }

    pub fn add_linear<T: Into<Offset>>(&mut self, a: T) {
        let a = self.compensate(a);
        self.position += a;
        self.contour.segments.push(Segment::Linear(a));
    }

    pub fn add_quadratic<T: Into<Offset>>(&mut self, a: T, b: T) {
        let (a, b) = (self.compensate(a), b.into());
        self.position += a;
        self.position += b;
        self.contour.segments.push(Segment::Quadratic(a, b));
    }

    pub fn compensate_by<T: Into<Offset>>(&mut self, value: T) {
        match &mut self.compensation {
            &mut Some(mut compensation) => compensation += value,
            compensation @ &mut None => *compensation = Some(value.into()),
        }
    }

    pub fn move_by<T: Into<Offset>>(&mut self, value: T) {
        self.flush();
        let value = self.compensate(value);
        self.position += value;
        self.start = self.position;
        self.contour.offset = value;
    }

    pub fn move_to_origin(&mut self) {
        self.flush();
        self.compensation = Some(-self.position);
    }

    #[inline]
    pub fn offset(&self) -> Offset {
        self.start - self.position
    }

    #[inline]
    pub fn position(&self) -> Offset {
        self.position
    }

    pub fn set_left_side_bearing<T: Into<Number>>(&mut self, value: T) {
        self.glyph.side_bearings.0 = value.into();
    }

    pub fn set_right_side_bearing<T: Into<Number>>(&mut self, value: T) {
        self.glyph.side_bearings.1 = value.into();
    }

    #[inline]
    pub fn set_min_x<T: Into<Number>>(&mut self, value: T) {
        self.glyph.bounding_box.0 = value.into();
    }

    #[inline]
    pub fn set_min_y<T: Into<Number>>(&mut self, value: T) {
        self.glyph.bounding_box.1 = value.into();
    }

    #[inline]
    pub fn set_max_x<T: Into<Number>>(&mut self, value: T) {
        self.glyph.bounding_box.2 = value.into();
    }

    #[inline]
    pub fn set_max_y<T: Into<Number>>(&mut self, value: T) {
        self.glyph.bounding_box.3 = value.into();
    }

    #[inline]
    fn compensate<T: Into<Offset>>(&mut self, value: T) -> Offset {
        if let Some(compensation) = self.compensation.take() {
            compensation + value
        } else {
            value.into()
        }
    }

    fn flush(&mut self) {
        if self.contour.is_empty() {
            return;
        }
        let offset = self.start - self.position;
        if offset != Offset::zero() {
            self.add_linear(offset);
        }
        self.glyph.contours.push(mem::replace(&mut self.contour, Default::default()));
    }
}

impl From<Builder> for Glyph {
    #[inline]
    fn from(mut builder: Builder) -> Glyph {
        builder.flush();
        builder.glyph
    }
}
