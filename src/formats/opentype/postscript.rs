use std::io::Result;

use opentype::postscript::compact1::FontSet;
use opentype::postscript::type2::Program;

use crate::formats::opentype::mapping::Forward as Mapping;
use crate::formats::opentype::metrics::Metrics;
use crate::glyph::{Builder, Glyph};
use crate::offset::Offset;

macro_rules! expect(
    ($condition:expr) => (
        if !$condition {
            raise!("found a malformed glyph");
        }
    )
);

pub(super) fn draw(
    font_set: &FontSet,
    mapping: &Mapping,
    metrics: &Metrics,
    id: usize,
    character: char,
) -> Result<Option<Glyph>> {
    use opentype::postscript::compact1::font_set::Record;
    use opentype::postscript::type2::Operator::*;

    let glyph_id = match mapping.get(character) {
        Some(glyph_id) => glyph_id,
        _ => return Ok(None),
    };
    let mut program = match font_set.character_strings[id].get(glyph_id as usize) {
        Some(character_string) => Program::new(
            character_string,
            &font_set.subroutines,
            match &font_set.records[id] {
                Record::CharacterNameKeyed(ref record) => &record.subroutines,
                _ => raise!("found a character-ID-keyed font, which is not supported yet"),
            },
        ),
        _ => raise!(
            "found no char string for character {} with glyph {}",
            character,
            glyph_id,
        ),
    };
    let mut builder = Builder::default();
    let mut position = Offset::default();
    let (mut max, mut min) = (Offset::undefined(), Offset::undefined());
    macro_rules! build(
        ($function:ident($(($x:expr, $y:expr)),+ $(,)?)) => (
            builder.$function($(($x, $y)),+);
            build!(@track $function($(($x, $y)),+));
        );
        (@track move_relative($(($x:expr, $y:expr)),+)) => (
            $(position += ($x, $y);)+
        );
        (@track $function:ident($(($x:expr, $y:expr)),+)) => (
            build!(@update);
            $(position += ($x, $y);)+
            build!(@update);
        );
        (@update) => (
            max = max.max(position);
            min = min.min(position);
        );
    );
    let mut clear = false;
    while let Some((operator, operands)) = program.next()? {
        let count = operands.len();
        match operator {
            RMoveTo | HMoveTo | VMoveTo => builder.flush(),
            _ => {}
        }
        match operator {
            RMoveTo => {
                expect!(count == 2 || !clear && count == 3);
                build!(move_relative((operands[0], operands[1])));
            }
            HMoveTo => {
                expect!(count == 1 || !clear && count == 2);
                build!(move_relative((operands[0], 0.0)));
            }
            VMoveTo => {
                expect!(count == 1 || !clear && count == 2);
                build!(move_relative((0.0, operands[0])));
            }
            RLineTo => {
                expect!(count % 2 == 0);
                for i in 0..(count / 2) {
                    let j = 2 * i;
                    build!(add_linear((operands[j], operands[j + 1])));
                }
            }
            HLineTo => {
                for (i, operand) in operands.iter().enumerate().take(count) {
                    if i % 2 == 0 {
                        build!(add_linear((*operand, 0.0)));
                    } else {
                        build!(add_linear((0.0, *operand)));
                    }
                }
            }
            VLineTo => {
                for (i, operand) in operands.iter().enumerate().take(count) {
                    if i % 2 == 1 {
                        build!(add_linear((*operand, 0.0)));
                    } else {
                        build!(add_linear((0.0, *operand)));
                    }
                }
            }
            RRCurveTo => {
                expect!(count % 6 == 0);
                for i in 0..(count / 6) {
                    let j = 6 * i;
                    build!(add_cubic(
                        (operands[j], operands[j + 1]),
                        (operands[j + 2], operands[j + 3]),
                        (operands[j + 4], operands[j + 5]),
                    ));
                }
            }
            HHCurveTo => {
                let (offset, first) = if count % 4 == 0 {
                    (0, 0.0)
                } else {
                    expect!((count - 1) % 4 == 0);
                    (1, operands[0])
                };
                for i in 0..((count - offset) / 4) {
                    let j = offset + 4 * i;
                    let first = if i == 0 { first } else { 0.0 };
                    build!(add_cubic(
                        (operands[j], first),
                        (operands[j + 1], operands[j + 2]),
                        (operands[j + 3], 0.0),
                    ));
                }
            }
            HVCurveTo => {
                let (steps, last) = if count % 4 == 0 {
                    (count / 4, 0.0)
                } else {
                    expect!((count - 1) % 4 == 0);
                    ((count - 1) / 4, operands[count - 1])
                };
                for i in 0..steps {
                    let j = 4 * i;
                    let last = if i + 1 == steps { last } else { 0.0 };
                    if i % 2 == 0 {
                        build!(add_cubic(
                            (operands[j], 0.0),
                            (operands[j + 1], operands[j + 2]),
                            (last, operands[j + 3]),
                        ));
                    } else {
                        build!(add_cubic(
                            (0.0, operands[j]),
                            (operands[j + 1], operands[j + 2]),
                            (operands[j + 3], last),
                        ));
                    }
                }
            }
            VHCurveTo => {
                let (steps, last) = if count % 4 == 0 {
                    (count / 4, 0.0)
                } else {
                    expect!((count - 1) % 4 == 0);
                    ((count - 1) / 4, operands[count - 1])
                };
                for i in 0..steps {
                    let j = 4 * i;
                    let last = if i + 1 == steps { last } else { 0.0 };
                    if i % 2 == 1 {
                        build!(add_cubic(
                            (operands[j], 0.0),
                            (operands[j + 1], operands[j + 2]),
                            (last, operands[j + 3]),
                        ));
                    } else {
                        build!(add_cubic(
                            (0.0, operands[j]),
                            (operands[j + 1], operands[j + 2]),
                            (operands[j + 3], last),
                        ));
                    }
                }
            }
            VVCurveTo => {
                let (offset, first) = if count % 4 == 0 {
                    (0, 0.0)
                } else {
                    expect!((count - 1) % 4 == 0);
                    (1, operands[0])
                };
                for i in 0..((count - offset) / 4) {
                    let j = offset + 4 * i;
                    let first = if i == 0 { first } else { 0.0 };
                    build!(add_cubic(
                        (first, operands[j]),
                        (operands[j + 1], operands[j + 2]),
                        (0.0, operands[j + 3]),
                    ));
                }
            }
            RCurveLine => {
                expect!(count >= 2 && (count - 2) % 6 == 0);
                for i in 0..((count - 2) / 6) {
                    let j = 6 * i;
                    build!(add_cubic(
                        (operands[j], operands[j + 1]),
                        (operands[j + 2], operands[j + 3]),
                        (operands[j + 4], operands[j + 5]),
                    ));
                }
                let j = count - 2;
                build!(add_linear((operands[j], operands[j + 1])));
            }
            RLineCurve => {
                expect!(count >= 6 && (count - 6) % 2 == 0);
                for i in 0..((count - 6) / 2) {
                    let j = 2 * i;
                    build!(add_linear((operands[j], operands[j + 1])));
                }
                let j = count - 6;
                build!(add_cubic(
                    (operands[j], operands[j + 1]),
                    (operands[j + 2], operands[j + 3]),
                    (operands[j + 4], operands[j + 5]),
                ));
            }
            CntrMask | HintMask | HStem | HStemHM | VStem | VStemHM => {}
            Flex | Flex1 | HFlex | HFlex1 => {}
            operator => raise!("found an unknown operation with operator {operator:?}"),
        }
        match operator {
            HMoveTo | VMoveTo | RMoveTo | HStem | HStemHM | VStem | VStemHM | CntrMask
            | HintMask => {
                clear = true;
            }
            _ => {}
        }
    }
    builder.flush();
    builder.set_bounding_box((min.0, min.1, max.0, max.1));
    builder.set_horizontal_metrics(metrics.get(glyph_id));
    Ok(Some(builder.into()))
}
