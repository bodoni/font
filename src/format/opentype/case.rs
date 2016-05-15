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
                    assert!(count == 2 || !clear && count == 3);
                    builder.move_to(operands[0].into(), operands[1].into());
                },
                HMoveTo => {
                    assert!(count == 1 || !clear && count == 2);
                    builder.move_to(operands[0].into(), 0.0);
                },
                VMoveTo => {
                    assert!(count == 1 || !clear && count == 2);
                    builder.move_to(0.0, operands[0].into());
                },
                RLineTo => {
                    assert!(count % 2 == 0);
                    for i in 0..(count >> 1) {
                        builder.line_to(operands[2 * i + 0].into(),
                                        operands[2 * i + 1].into());
                    }
                },
                HLineTo => {
                    for i in 0..count {
                        if i % 2 == 0 {
                            builder.line_to(operands[i].into(), 0.0);
                        } else {
                            builder.line_to(0.0, operands[i].into());
                        }
                    }
                },
                VLineTo => {
                    for i in 0..count {
                        if i % 2 == 0 {
                            builder.line_to(0.0, operands[i].into());
                        } else {
                            builder.line_to(operands[i].into(), 0.0);
                        }
                    }
                },
                RRCurveTo => {
                    assert!(count % 6 == 0);
                },
                HHCurveTo => {
                    if count % 4 == 0 {
                    } else {
                        assert!((count - 1) % 4 == 0);
                    }
                },
                HVCurveTo => {
                    if count % 8 == 0 {
                    } else if (count - 1) % 8 == 0 {
                    } else if (count - 4) % 8 == 0 {
                    } else {
                        assert!((count - 4 - 1) % 8 == 0);
                    }
                },
                VHCurveTo => {
                    if count % 8 == 0 {
                    } else if (count - 1) % 8 == 0 {
                    } else if (count - 4) % 8 == 0 {
                    } else {
                        assert!((count - 4 - 1) % 8 == 0);
                    }
                },
                VVCurveTo => {
                    if count % 4 == 0 {
                    } else {
                        assert!((count - 1) % 4 == 0);
                    }
                },
                RCurveLine => {
                    assert!(count >= 2 && (count - 2) % 6 == 0);
                },
                RLineCurve => {
                    assert!(count >= 6 && (count - 6) % 2 == 0);
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
