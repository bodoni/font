use std::rc::Rc;
use super::postscript::compact::FontSet;
use super::postscript::type2::Program;

use Result;
use glyph::{Builder, Glyph};
use super::mapping::Mapping;

pub struct PostScript {
    id: usize,
    fontset: Rc<FontSet>,
    mapping: Rc<Mapping>,
}

impl PostScript {
    #[inline]
    pub fn new(id: usize, fontset: Rc<FontSet>, mapping: Rc<Mapping>) -> PostScript {
        PostScript { id: id, fontset: fontset, mapping: mapping }
    }
}

impl ::case::Case for PostScript {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        use super::postscript::type2::Operator::*;

        macro_rules! expect(
            ($condition:expr) => (assert!($condition));
        );

        let mut program = {
            let id = match self.mapping.find(glyph) {
                Some(id) => id,
                _ => return Ok(None),
            };
            Program::new(&self.fontset.char_strings[self.id][id],
                         &self.fontset.global_subroutines,
                         &self.fontset.local_subroutines[self.id])
        };

        let mut builder = Builder::new();

        let mut clear = false;
        while let Some((operator, operands)) = try!(program.next()) {
            let count = operands.len();
            match operator {
                RMoveTo => {
                    expect!(count == 2 || !clear && count == 3);
                    builder.move_to((operands[0].into(), operands[1].into()));
                },
                HMoveTo => {
                    expect!(count == 1 || !clear && count == 2);
                    builder.move_to((operands[0].into(), 0.0));
                },
                VMoveTo => {
                    expect!(count == 1 || !clear && count == 2);
                    builder.move_to((0.0, operands[0].into()));
                },
                RLineTo => {
                    expect!(count % 2 == 0);
                    for i in 0..(count / 2) {
                        let j = 2 * i;
                        builder.line_to((operands[j + 0].into(), operands[j + 1].into()));
                    }
                },
                HLineTo => {
                    for i in 0..count {
                        if i % 2 == 0 {
                            builder.line_to((operands[i].into(), 0.0));
                        } else {
                            builder.line_to((0.0, operands[i].into()));
                        }
                    }
                },
                VLineTo => {
                    for i in 0..count {
                        if i % 2 == 0 {
                            builder.line_to((0.0, operands[i].into()));
                        } else {
                            builder.line_to((operands[i].into(), 0.0));
                        }
                    }
                },
                RRCurveTo => {
                    expect!(count % 6 == 0);
                    for i in 0..(count / 6) {
                        let j = 6 * i;
                        builder.bezier_to(
                            (operands[j + 0].into(), operands[j + 1].into()),
                            (operands[j + 2].into(), operands[j + 3].into()),
                            (operands[j + 4].into(), operands[j + 5].into()),
                        );
                    }
                },
                HHCurveTo => {
                    let (offset, mut extra) = if count % 4 == 0 {
                        (0, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        (1, operands[0].into())
                    };
                    for i in 0..((count - offset) / 4) {
                        let j = offset + 4 * i;
                        builder.bezier_to(
                            (operands[j + 0].into(), extra),
                            (operands[j + 1].into(), operands[j + 2].into()),
                            (operands[j + 3].into(), 0.0),
                        );
                        extra = 0.0;
                    }
                },
                HVCurveTo => {
                    if count % 8 == 0 {
                    } else if (count - 1) % 8 == 0 {
                    } else if (count - 4) % 8 == 0 {
                    } else {
                        expect!((count - 4 - 1) % 8 == 0);
                    }
                },
                VHCurveTo => {
                    if count % 8 == 0 {
                    } else if (count - 1) % 8 == 0 {
                    } else if (count - 4) % 8 == 0 {
                    } else {
                        expect!((count - 4 - 1) % 8 == 0);
                    }
                },
                VVCurveTo => {
                    let (offset, mut extra) = if count % 4 == 0 {
                        (0, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        (1, operands[0].into())
                    };
                    for i in 0..((count - offset) / 4) {
                        let j = offset + 4 * i;
                        builder.bezier_to(
                            (extra, operands[j + 0].into()),
                            (operands[j + 1].into(), operands[j + 2].into()),
                            (0.0, operands[j + 3].into()),
                        );
                        extra = 0.0;
                    }
                },
                RCurveLine => {
                    expect!(count >= 2 && (count - 2) % 6 == 0);
                    for i in 0..((count - 2) / 6) {
                        let j = 6 * i;
                        builder.bezier_to(
                            (operands[j + 0].into(), operands[j + 1].into()),
                            (operands[j + 2].into(), operands[j + 3].into()),
                            (operands[j + 4].into(), operands[j + 5].into()),
                        );
                    }
                    let j = count - 2;
                    builder.line_to((operands[j + 0].into(), operands[j + 1].into()));
                },
                RLineCurve => {
                    expect!(count >= 6 && (count - 6) % 2 == 0);
                    for i in 0..((count - 6) / 2) {
                        let j = 2 * i;
                        builder.line_to((operands[j + 0].into(), operands[j + 1].into()));
                    }
                    let j = count - 6;
                    builder.bezier_to(
                        (operands[j + 0].into(), operands[j + 1].into()),
                        (operands[j + 2].into(), operands[j + 3].into()),
                        (operands[j + 4].into(), operands[j + 5].into()),
                    );
                },
                HStem | HStemHM | VStem | VStemHM | CntrMask | HintMask => {},
                _ => unreachable!(),
            }
            match operator {
                HMoveTo | VMoveTo | RMoveTo |
                HStem | HStemHM | VStem | VStemHM |
                CntrMask | HintMask => {
                    clear = true;
                },
                _ => {},
            }
        }

        Ok(Some(builder.into()))
    }
}
