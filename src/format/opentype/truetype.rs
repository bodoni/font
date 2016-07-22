use std::rc::Rc;
use truetype::glyph_data::{Compound, GlyphData, Simple};

use {Offset, Result};
use case::Case;
use glyph::{Builder, Glyph};
use super::mapping::Mapping;

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    mapping: Rc<Mapping>,
}

macro_rules! expect(($condition:expr) => (if !$condition { reject!(); }));
macro_rules! reject(() => (raise!("found a malformed glyph")));

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, mapping: Rc<Mapping>) -> Self {
        TrueType { glyph_data: glyph_data, mapping: mapping }
    }

    fn draw_index(&self, builder: &mut Builder, index: usize) -> Result<()> {
        use truetype::glyph_data::Description::*;
        match self.glyph_data.get(index) {
            Some(&Some(ref glyph)) => match &glyph.description {
                &Simple(ref description) => draw_simple(builder, description),
                &Compound(ref description) => draw_compound(self, builder, description),
            },
            Some(&None) => return Ok(()),
            _ => reject!(),
        }
    }
}

impl Case for TrueType {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        let mut builder = Builder::new();
        if let Some(index) = self.mapping.find(glyph) {
            try!(self.draw_index(&mut builder, index));
        } else {
            return Ok(None);
        }
        Ok(Some(builder.into()))
    }
}

fn draw_simple(builder: &mut Builder, description: &Simple) -> Result<()> {
    let &Simple { ref end_points, ref flags, ref x, ref y, .. } = description;

    let point_count = flags.len();
    expect!(point_count > 0 && point_count == x.len() && point_count == y.len());

    macro_rules! is_on_curve(($i:expr) => (flags[$i].is_on_curve()));
    macro_rules! read(($i:expr) => (Offset::from((x[$i], y[$i]))));

    let mut cursor = 0;
    for &end in end_points {
        let end = end as usize;
        expect!(end < point_count && is_on_curve!(cursor));
        builder.move_by(read!(cursor));
        let mut control: Option<Offset> = None;
        for cursor in (cursor + 1)..(end + 1) {
            let current = read!(cursor);
            if is_on_curve!(cursor) {
                match control.take() {
                    Some(control) => builder.quadratic_by(control, current),
                    _ => builder.linear_by(current),
                }
            } else {
                match &mut control {
                    &mut Some(ref mut control) => {
                        let half = current / 2.0;
                        builder.quadratic_by(*control, half);
                        *control = half;
                    },
                    control @ &mut None => {
                        *control = Some(current);
                    },
                }
            }
        }
        if let Some(control) = control.take() {
            let current = builder.offset() - control;
            builder.quadratic_by(control, current);
            builder.compensate_by(-current);
        }
        cursor = end + 1;
    }

    Ok(())
}

fn draw_compound(case: &TrueType, builder: &mut Builder, description: &Compound) -> Result<()> {
    use truetype::glyph_data::{Arguments, Options};

    for component in description.components.iter() {
        builder.move_to_origin();
        match &component.arguments {
            &Arguments::Offsets(x, y) => builder.compensate_by((x, y)),
            &Arguments::Indices(..) => unimplemented!(),
        }
        match &component.options {
            &Options::None => {},
            &Options::Scalar(..) => unimplemented!(),
            &Options::Vector(..) => unimplemented!(),
            &Options::Matrix(..) => unimplemented!(),
        }
        try!(case.draw_index(builder, component.index as usize));
    }
    Ok(())
}
