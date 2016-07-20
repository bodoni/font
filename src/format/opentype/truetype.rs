use std::rc::Rc;
use truetype::glyph_data::{GlyphData, Simple};

use Result;
use case::Case;
use glyph::{Builder, Glyph};
use super::mapping::Mapping;

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    mapping: Rc<Mapping>,
}

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, mapping: Rc<Mapping>) -> Self {
        TrueType { glyph_data: glyph_data, mapping: mapping }
    }
}

macro_rules! expect(($condition:expr) => (if !$condition { reject!(); }));
macro_rules! reject(() => (raise!("found a malformed glyph")));

impl Case for TrueType {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        use truetype::glyph_data::Description::*;

        let glyph = match self.mapping.find(glyph) {
            Some(id) => match self.glyph_data.get(id) {
                Some(&Some(ref glyph)) => glyph,
                Some(&None) => return Ok(Some(Default::default())),
                _ => reject!(),
            },
            _ => return Ok(None),
        };
        match &glyph.description {
            &Simple(ref description) => draw_simple(description),
            &Compound(..) => unimplemented!(),
        }
    }
}

fn draw_simple(&Simple { ref end_points, ref flags, ref x, ref y, .. }: &Simple)
               -> Result<Option<Glyph>> {

    macro_rules! is_control(($i:expr) => (flags[$i] & 0b1 == 0));
    macro_rules! read(($i:expr) => ((x[$i] as f32, y[$i] as f32)));

    let point_count = flags.len();
    expect!(point_count > 0 && point_count == x.len() && point_count == y.len());

    let mut builder = Builder::new();

    let mut cursor = 0;
    for &end in end_points {
        let end = end as usize;
        expect!(end < point_count && !is_control!(cursor));
        let first = read!(cursor);
        builder.move_to(first);
        let mut control: Option<(f32, f32)> = None;
        for cursor in (cursor + 1)..(end + 1) {
            let current = read!(cursor);
            if is_control!(cursor) {
                match &mut control {
                    &mut Some(ref mut control) => {
                        let half = (current.0 / 2.0, current.1 / 2.0);
                        builder.quadratic_curve_to(*control, Some(half));
                        *control = half;
                    },
                    control @ &mut None => {
                        *control = Some(current);
                    },
                }
            } else {
                match control.take() {
                    Some(control) => builder.quadratic_curve_to(control, Some(current)),
                    _ => builder.line_to(current),
                }
            }
        }
        if let Some(control) = control.take() {
            builder.quadratic_curve_to(control, None);
        }
        cursor = end + 1;
    }

    Ok(Some(builder.into()))
}
