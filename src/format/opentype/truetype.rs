use std::rc::Rc;
use truetype::glyph_data::{self, CompositeDescription, GlyphData, SimpleDescription};

use super::mapping::Mapping;
use super::metrics::Metrics;
use crate::builder::Builder;
use crate::case::Case;
use crate::glyph::Glyph;
use crate::{Offset, Result};

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    metrics: Rc<Metrics>,
    mapping: Rc<Mapping>,
}

macro_rules! expect(($condition:expr) => (if !$condition { reject!(); }));
macro_rules! reject(() => (raise!("found a malformed glyph")));

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, metrics: Rc<Metrics>, mapping: Rc<Mapping>) -> Self {
        TrueType {
            glyph_data: glyph_data,
            metrics: metrics,
            mapping: mapping,
        }
    }

    fn draw_glyph(&self, builder: &mut Builder, glyph: &glyph_data::Glyph) -> Result<()> {
        use truetype::glyph_data::Description::*;

        match &glyph.description {
            &Simple(ref description) => draw_simple(builder, description),
            &Composite(ref description) => draw_composite(self, builder, description),
        }
    }
}

impl Case for TrueType {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        let mut builder = Builder::new();
        let glyph_index = match self.mapping.find(glyph) {
            Some(glyph_index) => glyph_index,
            _ => return Ok(None),
        };
        let glyph = match self.glyph_data.get(glyph_index) {
            Some(glyph) => glyph,
            _ => reject!(),
        };
        builder.set_horizontal_metrics(self.metrics.get(glyph_index));
        if let &Some(ref glyph) = glyph {
            self.draw_glyph(&mut builder, glyph)?;
            builder.set_bounding_box(glyph.min_x, glyph.min_y, glyph.max_x, glyph.max_y);
        }
        Ok(Some(builder.into()))
    }
}

fn draw_simple(builder: &mut Builder, description: &SimpleDescription) -> Result<()> {
    let &SimpleDescription {
        ref end_points,
        ref flags,
        ref x,
        ref y,
        ..
    } = description;
    let point_count = flags.len();
    expect!(point_count > 0 && point_count == x.len() && point_count == y.len());
    macro_rules! is_on_curve(($i:expr) => (flags[$i].is_on_curve()));
    macro_rules! read(($i:expr) => (Offset::from((x[$i], y[$i]))));
    let mut cursor = 0;
    for &end in end_points {
        let end = end as usize;
        expect!(end < point_count && is_on_curve!(cursor));
        builder.jump(read!(cursor));
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
                    }
                    control @ &mut None => {
                        *control = Some(current);
                    }
                }
            }
        }
        if let Some(control) = control.take() {
            let current = builder.offset() - control;
            builder.add_quadratic(control, current);
            builder.add_compensation(-current);
        }
        cursor = end + 1;
    }
    Ok(())
}

fn draw_composite(
    case: &TrueType,
    builder: &mut Builder,
    description: &CompositeDescription,
) -> Result<()> {
    use truetype::glyph_data::{Arguments, Options};

    for component in description.components.iter() {
        let glyph_index = component.glyph_index as usize;
        builder.restart();
        match &component.arguments {
            &Arguments::Offsets(x, y) => builder.add_compensation((x, y)),
            &Arguments::Indices(..) => unimplemented!(),
        }
        match &component.options {
            &Options::None => {}
            &Options::Scalar(..) => unimplemented!(),
            &Options::Vector(..) => unimplemented!(),
            &Options::Matrix(..) => unimplemented!(),
        }
        let glyph = match case.glyph_data.get(glyph_index) {
            Some(&Some(ref glyph)) => glyph,
            Some(&None) => continue,
            _ => reject!(),
        };
        if component.flags.should_use_metrics() {
            builder.set_horizontal_metrics(case.metrics.get(glyph_index));
        }
        case.draw_glyph(builder, glyph)?;
    }
    Ok(())
}
