use std::mem;

use {Number, Offset};
use glyph::{Contour, Glyph, Segment};

#[derive(Default)]
pub struct Builder {
    start: Offset,
    position: Offset,
    contour: Contour,
    glyph: Glyph,
    advance_width: Number,
    compensation: Option<Offset>,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn offset(&self) -> Offset {
        self.start - self.position
    }

    #[inline]
    pub fn position(&self) -> Offset {
        self.position
    }

    fn flush(&mut self) {
        if self.contour.is_empty() {
            return;
        }
        let offset = self.start - self.position;
        if offset != Offset::default() {
            self.add_linear(offset);
        }
        self.glyph.contours.push(mem::replace(&mut self.contour, Default::default()));
    }
}

impl Builder {
    pub fn add_cubic<T: Into<Offset>>(&mut self, a: T, b: T, c: T) {
        let (a, b, c) = (self.use_compensation(a), b.into(), c.into());
        self.add_segment(Segment::Cubic(a, b, c), a + b + c);
    }

    pub fn add_linear<T: Into<Offset>>(&mut self, a: T) {
        let a = self.use_compensation(a);
        self.add_segment(Segment::Linear(a), a);
    }

    pub fn add_quadratic<T: Into<Offset>>(&mut self, a: T, b: T) {
        let (a, b) = (self.use_compensation(a), b.into());
        self.add_segment(Segment::Quadratic(a, b), a + b);
    }

    pub fn jump<T: Into<Offset>>(&mut self, a: T) {
        self.flush();
        let a = self.use_compensation(a);
        self.position += a;
        self.contour.offset = a;
        self.start = self.position;
    }

    pub fn restart(&mut self) {
        self.flush();
        self.compensation = Some(-self.position);
    }

    #[inline]
    fn add_segment(&mut self, segment: Segment, offset: Offset) {
        self.track_bounding_box();
        self.position += offset;
        self.track_bounding_box();
        self.contour.segments.push(segment);
    }
}

impl Builder {
    pub fn add_compensation<T: Into<Offset>>(&mut self, value: T) {
        match &mut self.compensation {
            &mut Some(mut compensation) => compensation += value,
            compensation @ &mut None => *compensation = Some(value.into()),
        }
    }

    #[inline]
    fn use_compensation<T: Into<Offset>>(&mut self, value: T) -> Offset {
        if let Some(compensation) = self.compensation.take() {
            compensation + value
        } else {
            value.into()
        }
    }
}

impl Builder {
    #[inline]
    pub fn set_bounding_box<T: Into<Number>>(&mut self, min_x: T, min_y: T, max_x: T, max_y: T) {
        self.glyph.bounding_box = (min_x.into(), min_y.into(), max_x.into(), max_y.into());
    }

    #[inline]
    pub fn set_horizontal_metrics(&mut self, metrics: (Number, Number)) {
        self.advance_width = metrics.0;
        self.glyph.side_bearings.0 = metrics.1;
    }

    #[inline]
    fn track_bounding_box(&mut self) {
        let bounding_box = &mut self.glyph.bounding_box;
        bounding_box.0 = bounding_box.0.min(self.position.0);
        bounding_box.1 = bounding_box.1.min(self.position.1);
        bounding_box.2 = bounding_box.2.max(self.position.0);
        bounding_box.3 = bounding_box.3.max(self.position.1);
    }
}

impl From<Builder> for Glyph {
    fn from(mut builder: Builder) -> Glyph {
        builder.flush();
        let Builder { mut glyph, advance_width, .. } = builder;
        let width = glyph.bounding_box.2 - glyph.bounding_box.0;
        glyph.side_bearings.1 = advance_width - (glyph.side_bearings.0 + width);
        glyph
    }
}
