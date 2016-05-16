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
    pub fn new(id: usize, fontset: Rc<FontSet>, mapping: Rc<Mapping>) -> Self {
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
            macro_rules! get(($index:expr) => (operands[$index].into()));
            let count = operands.len();
            match operator {
                RMoveTo => {
                    expect!(count == 2 || !clear && count == 3);
                    builder.move_to((get!(0), get!(1)));
                },
                HMoveTo => {
                    expect!(count == 1 || !clear && count == 2);
                    builder.move_to((get!(0), 0.0));
                },
                VMoveTo => {
                    expect!(count == 1 || !clear && count == 2);
                    builder.move_to((0.0, get!(0)));
                },
                RLineTo => {
                    expect!(count % 2 == 0);
                    for i in 0..(count / 2) {
                        let j = 2 * i;
                        builder.line_to((get!(j + 0), get!(j + 1)));
                    }
                },
                HLineTo => {
                    for i in 0..count {
                        if i % 2 == 0 {
                            builder.line_to((get!(i), 0.0));
                        } else {
                            builder.line_to((0.0, get!(i)));
                        }
                    }
                },
                VLineTo => {
                    for i in 0..count {
                        if i % 2 == 1 {
                            builder.line_to((get!(i), 0.0));
                        } else {
                            builder.line_to((0.0, get!(i)));
                        }
                    }
                },
                RRCurveTo => {
                    expect!(count % 6 == 0);
                    for i in 0..(count / 6) {
                        let j = 6 * i;
                        builder.curve_to(
                            (get!(j + 0), get!(j + 1)),
                            (get!(j + 2), get!(j + 3)),
                            (get!(j + 4), get!(j + 5)),
                        );
                    }
                },
                HHCurveTo => {
                    let (offset, first) = if count % 4 == 0 {
                        (0, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        (1, get!(0))
                    };
                    for i in 0..((count - offset) / 4) {
                        let j = offset + 4 * i;
                        let first = if i == 0 { first } else { 0.0 };
                        builder.curve_to(
                            (get!(j + 0),       first),
                            (get!(j + 1), get!(j + 2)),
                            (get!(j + 3),         0.0),
                        );
                    }
                },
                HVCurveTo => {
                    let (steps, last) = if count % 4 == 0 {
                        (count / 4, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        ((count - 1) / 4, get!(count - 1))
                    };
                    for i in 0..steps {
                        let j = 4 * i;
                        let last = if i + 1 == steps { last } else { 0.0 };
                        if i % 2 == 0 {
                            builder.curve_to(
                                (get!(j + 0),         0.0),
                                (get!(j + 1), get!(j + 2)),
                                (       last, get!(j + 3)),
                            );
                        } else {
                            builder.curve_to(
                                (        0.0, get!(j + 0)),
                                (get!(j + 1), get!(j + 2)),
                                (get!(j + 3),        last),
                            );
                        }
                    }
                },
                VHCurveTo => {
                    let (steps, last) = if count % 4 == 0 {
                        (count / 4, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        ((count - 1) / 4, get!(count - 1))
                    };
                    for i in 0..steps {
                        let j = 4 * i;
                        let last = if i + 1 == steps { last } else { 0.0 };
                        if i % 2 == 1 {
                            builder.curve_to(
                                (get!(j + 0),         0.0),
                                (get!(j + 1), get!(j + 2)),
                                (       last, get!(j + 3)),
                            );
                        } else {
                            builder.curve_to(
                                (        0.0, get!(j + 0)),
                                (get!(j + 1), get!(j + 2)),
                                (get!(j + 3),        last),
                            );
                        }
                    }
                },
                VVCurveTo => {
                    let (offset, first) = if count % 4 == 0 {
                        (0, 0.0)
                    } else {
                        expect!((count - 1) % 4 == 0);
                        (1, get!(0))
                    };
                    for i in 0..((count - offset) / 4) {
                        let j = offset + 4 * i;
                        let first = if i == 0 { first } else { 0.0 };
                        builder.curve_to(
                            (      first, get!(j + 0)),
                            (get!(j + 1), get!(j + 2)),
                            (        0.0, get!(j + 3)),
                        );
                    }
                },
                RCurveLine => {
                    expect!(count >= 2 && (count - 2) % 6 == 0);
                    for i in 0..((count - 2) / 6) {
                        let j = 6 * i;
                        builder.curve_to(
                            (get!(j + 0), get!(j + 1)),
                            (get!(j + 2), get!(j + 3)),
                            (get!(j + 4), get!(j + 5)),
                        );
                    }
                    let j = count - 2;
                    builder.line_to((get!(j + 0), get!(j + 1)));
                },
                RLineCurve => {
                    expect!(count >= 6 && (count - 6) % 2 == 0);
                    for i in 0..((count - 6) / 2) {
                        let j = 2 * i;
                        builder.line_to((get!(j + 0), get!(j + 1)));
                    }
                    let j = count - 6;
                    builder.curve_to(
                        (get!(j + 0), get!(j + 1)),
                        (get!(j + 2), get!(j + 3)),
                        (get!(j + 4), get!(j + 5)),
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
