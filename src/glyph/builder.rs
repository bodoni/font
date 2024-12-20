use crate::glyph::{Contour, Glyph, Segment};
use crate::{Number, Offset};

pub struct Builder {
    contour: Contour,
    glyph: Glyph,

    offset: Offset,
    scale: (Number, Number, Number, Number),
}

impl Builder {
    pub fn flush(&mut self) {
        if self.contour.is_empty() {
            return;
        }
        self.glyph.contours.push(std::mem::take(&mut self.contour));
    }

    pub fn nest<T, U, V, F>(&mut self, offset: T, scale: (U, U, U, U), build: F) -> V
    where
        T: Into<Offset>,
        U: Into<Number>,
        F: Fn(&mut Builder) -> V,
    {
        macro_rules! multiply(
            ($product:expr, $left:expr, $right:expr) => (
                $product.0 = $left.0 * $right.0 + $left.1 * $right.2;
                $product.1 = $left.0 * $right.1 + $left.1 * $right.3;
                $product.2 = $left.2 * $right.0 + $left.3 * $right.2;
                $product.3 = $left.2 * $right.1 + $left.3 * $right.3;
            )
        );

        let offset = offset.into();
        let scale = (
            scale.0.into(),
            scale.1.into(),
            scale.2.into(),
            scale.3.into(),
        );
        let previous_offset = self.offset;
        let previous_scale = self.scale;
        self.offset += offset;
        multiply!(self.scale, scale, self.scale);
        let result = build(self);
        self.scale = previous_scale;
        self.offset = previous_offset;
        result
    }

    #[inline]
    pub fn transform<T: Into<Offset>>(&self, value: T) -> Offset {
        macro_rules! multiply(
            ($product:expr, $left:expr, $right:expr) => (
                $product.0 = $left.0 * $right.0 + $left.1 * $right.1;
                $product.1 = $left.2 * $right.0 + $left.3 * $right.1;
            )
        );

        let mut value = value.into();
        multiply!(value, self.scale, value);
        value
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
            _ => return,
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
            scale: (1.0, 0.0, 0.0, 1.0),
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
