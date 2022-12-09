use postscript::compact1::FontSet;
use postscript::type2::Program;
use std::rc::Rc;

use super::mapping::Mapping;
use super::metrics::Metrics;
use crate::builder::Builder;
use crate::case::Case;
use crate::glyph::Glyph;
use crate::Result;

pub struct PostScript {
    id: usize,
    font_set: Rc<FontSet>,
    metrics: Rc<Metrics>,
    mapping: Rc<Mapping>,
}

impl PostScript {
    #[inline]
    pub fn new(
        id: usize,
        font_set: Rc<FontSet>,
        metrics: Rc<Metrics>,
        mapping: Rc<Mapping>,
    ) -> Self {
        PostScript {
            id: id,
            font_set: font_set,
            metrics: metrics,
            mapping: mapping,
        }
    }
}

impl Case for PostScript {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        use postscript::compact1::font_set::Record;
        use postscript::type2::Operator::*;

        macro_rules! expect(
            ($condition:expr) => (
                if !$condition {
                    raise!("found a malformed glyph");
                }
            )
        );

        let glyph_index = match self.mapping.find(glyph) {
            Some(glyph_index) => glyph_index,
            _ => return Ok(None),
        };
        let mut program = match self.font_set.char_strings[self.id].get(glyph_index as usize) {
            Some(char_string) => Program::new(
                char_string,
                &self.font_set.subroutines,
                match &self.font_set.records[self.id] {
                    Record::CharacterNameKeyed(ref record) => &*record.subroutines,
                    _ => unimplemented!(),
                },
            ),
            _ => raise!("found no char string for glyph {}", glyph),
        };
        let mut builder = Builder::new();
        macro_rules! build(
            ($function:ident($(($x:expr, $y:expr)),+ $(,)?)) => (
                builder.$function($(($x, $y)),+);
            )
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
                    for i in 0..count {
                        if i % 2 == 0 {
                            build!(add_linear((operands[i], 0.0)));
                        } else {
                            build!(add_linear((0.0, operands[i])));
                        }
                    }
                }
                VLineTo => {
                    for i in 0..count {
                        if i % 2 == 1 {
                            build!(add_linear((operands[i], 0.0)));
                        } else {
                            build!(add_linear((0.0, operands[i])));
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
                HStem | HStemHM | VStem | VStemHM | CntrMask | HintMask => {}
                _ => unreachable!(),
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
        builder.set_horizontal_metrics(self.metrics.get(glyph_index));
        Ok(Some(builder.into()))
    }
}
