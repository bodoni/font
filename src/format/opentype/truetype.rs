use std::rc::Rc;

use truetype::glyph_data::{self, CompositeDescription, GlyphData, SimpleDescription};

use super::mapping::Mapping;
use super::metrics::Metrics;
use crate::glyph::{Builder, Glyph};
use crate::offset::Offset;
use crate::Result;

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    metrics: Rc<Metrics>,
    mapping: Rc<Mapping>,
}

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, metrics: Rc<Metrics>, mapping: Rc<Mapping>) -> Self {
        TrueType {
            glyph_data: glyph_data,
            metrics: metrics,
            mapping: mapping,
        }
    }

    pub fn draw(&self, character: char) -> Result<Option<Glyph>> {
        let mut builder = Builder::default();
        let glyph_id = match self.mapping.find(character) {
            Some(glyph_id) => glyph_id,
            _ => return Ok(None),
        };
        let glyph = match self.glyph_data.get(glyph_id as usize) {
            Some(glyph) => glyph,
            _ => raise!(
                "found no data for character {} with glyph {}",
                character,
                glyph_id,
            ),
        };
        builder.set_horizontal_metrics(self.metrics.get(glyph_id));
        if let &Some(ref glyph) = glyph {
            self.draw_glyph(&mut builder, glyph)?;
            builder.set_bounding_box((glyph.min_x, glyph.min_y, glyph.max_x, glyph.max_y));
        }
        Ok(Some(builder.into()))
    }

    fn draw_glyph(&self, builder: &mut Builder, glyph: &glyph_data::Glyph) -> Result<()> {
        use truetype::glyph_data::Description::*;

        match &glyph.description {
            &Simple(ref description) => draw_simple(builder, description),
            &Composite(ref description) => draw_composite(self, builder, description),
        }
    }
}

fn draw_simple(builder: &mut Builder, description: &SimpleDescription) -> Result<()> {
    macro_rules! expect(
        ($condition:expr) => (
            if !$condition {
                raise!("found a malformed glyph");
            }
        )
    );

    let &SimpleDescription {
        ref end_points,
        ref flags,
        ref x,
        ref y,
        ..
    } = description;
    let point_count = flags.len();
    expect!(point_count == x.len());
    expect!(point_count == y.len());
    let mut i = 0;
    let mut sum = Offset::default();
    for k in end_points.iter().map(|&k| k as usize) {
        expect!(i < point_count);
        let start = Offset::from((x[i], y[i]));
        let mut control = match flags[i].is_on_curve() {
            false => Some(Offset::default()),
            _ => None,
        };
        let mut sum_delta = start;
        let mut offset = Offset::default();
        for j in (i + 1)..=k {
            expect!(j < point_count);
            let current = (x[j], y[j]).into();
            sum_delta += current;
            match (flags[j].is_on_curve(), &mut control) {
                (false, control @ &mut None) => {
                    *control = Some(current);
                }
                (false, &mut Some(ref mut control)) => {
                    let current = current / 2.0;
                    builder.add_quadratic(*control, current);
                    offset += *control + current;
                    *control = current;
                }
                (true, &mut None) => {
                    builder.add_linear(current);
                    offset += current;
                }
                (true, control @ &mut Some(_)) => {
                    let control = control.take().unwrap();
                    builder.add_quadratic(control, current);
                    offset += control + current;
                }
            }
        }
        match (flags[i].is_on_curve(), control) {
            (false, None) => {
                let control = (sum + start) - (sum + sum_delta);
                builder.move_control(control);
                offset += control;
                builder.move_absolute(sum + sum_delta);
            }
            (false, Some(control)) => {
                let current = ((sum + start) - (sum + sum_delta)) / 2.0;
                if !control.is_zero() || !current.is_zero() {
                    builder.add_quadratic(control, current);
                }
                offset += control + current;
                builder.move_control(current);
                offset += current;
                builder.move_absolute(sum + sum_delta + current);
            }
            (true, None) => {
                let current = -offset;
                if !current.is_zero() {
                    builder.add_linear(current);
                }
                offset += current;
                builder.move_absolute(sum + start);
            }
            (true, Some(control)) => {
                let current = -offset - control;
                if !control.is_zero() || !current.is_zero() {
                    builder.add_quadratic(control, current);
                }
                offset += control + current;
                builder.move_absolute(sum + start);
            }
        }
        debug_assert!(offset.is_zero());
        builder.flush();
        sum += sum_delta;
        i = k + 1;
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
        let glyph_id = component.glyph_id;
        let offset = match &component.arguments {
            &Arguments::Offsets(x, y) => Offset::from((x, y)),
            arguments => raise!(
                "found an unsupported component with arguments {:?}",
                arguments,
            ),
        };
        let scale = match &component.options {
            &Options::None => (1.0, 0.0, 0.0, 1.0),
            &Options::Scalar(value) => (value.into(), 0.0, 0.0, value.into()),
            &Options::Vector(x, y) => (x.into(), 0.0, 0.0, y.into()),
            &Options::Matrix(xx, xy, yx, yy) => (xx.into(), xy.into(), yx.into(), yy.into()),
        };
        let glyph = match case.glyph_data.get(glyph_id as usize) {
            Some(&Some(ref glyph)) => glyph,
            Some(&None) => continue,
            _ => raise!("found no data for glyph {}", glyph_id),
        };
        if component.flags.should_use_metrics() {
            builder.set_horizontal_metrics(case.metrics.get(glyph_id));
        }
        builder.nest(offset, scale, |builder| case.draw_glyph(builder, glyph))?;
    }
    Ok(())
}
