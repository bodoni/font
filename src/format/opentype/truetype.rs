use std::rc::Rc;
use truetype::glyph_data::{self, Compound, GlyphData, Simple};

use {Offset, Result};
use builder::Builder;
use case::Case;
use glyph::Glyph;
use super::mapping::Mapping;
use super::metrics::Metrics;

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    mapping: Rc<Mapping>,
}

macro_rules! expect(($condition:expr) => (if !$condition { reject!(); }));
macro_rules! reject(() => (raise!("found a malformed glyph")));

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, _: Rc<Metrics>, mapping: Rc<Mapping>) -> Self {
        TrueType { glyph_data: glyph_data, mapping: mapping }
    }

    fn draw_glyph(&self, builder: &mut Builder, glyph: &glyph_data::Glyph) -> Result<()> {
        use truetype::glyph_data::Description::*;
        match &glyph.description {
            &Simple(ref description) => draw_simple(builder, description),
            &Compound(ref description) => draw_compound(self, builder, description),
        }
    }
}

impl Case for TrueType {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        let mut builder = Builder::new();
        let index = match self.mapping.find(glyph) {
            Some(index) => index,
            _ => return Ok(None),
        };
        let glyph = match self.glyph_data.get(index) {
            Some(glyph) => glyph,
            _ => reject!(),
        };
        if let &Some(ref glyph) = glyph {
            builder.set_max_x(glyph.max_x);
            builder.set_max_y(glyph.max_y);
            builder.set_min_x(glyph.min_x);
            builder.set_min_y(glyph.min_y);
            try!(self.draw_glyph(&mut builder, glyph));
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
                    Some(control) => builder.add_quadratic(control, current),
                    _ => builder.add_linear(current),
                }
            } else {
                match &mut control {
                    &mut Some(ref mut control) => {
                        let half = current / 2.0;
                        builder.add_quadratic(*control, half);
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
            builder.add_quadratic(control, current);
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
        let glyph = match case.glyph_data.get(component.index as usize) {
            Some(&Some(ref glyph)) => glyph,
            Some(&None) => continue,
            _ => reject!(),
        };
        if component.flags.should_use_metrics() {
            builder.set_max_x(glyph.max_x);
            builder.set_max_y(glyph.max_y);
            builder.set_min_x(glyph.min_x);
            builder.set_min_y(glyph.min_y);
        }
        try!(case.draw_glyph(builder, glyph));
    }
    Ok(())
}
