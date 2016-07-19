use postscript::compact::FontSet;
use postscript::type2::Program;
use std::rc::Rc;
use truetype::GlyphData;

use Result;
use case::Case;
use glyph::{Builder, Glyph};
use super::mapping::Mapping;

pub struct PostScript {
    id: usize,
    font_set: Rc<FontSet>,
    mapping: Rc<Mapping>,
}

pub struct TrueType {
    glyph_data: Rc<GlyphData>,
    mapping: Rc<Mapping>,
}

impl PostScript {
    #[inline]
    pub fn new(id: usize, font_set: Rc<FontSet>, mapping: Rc<Mapping>) -> Self {
        PostScript { id: id, font_set: font_set, mapping: mapping }
    }
}

impl Case for PostScript {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        use postscript::type2::operation::Operator::*;

        macro_rules! expect(($condition:expr) => (if !$condition { reject!(); }));
        macro_rules! reject(() => (raise!("found a malformed glyph")));

        let mut program = match self.mapping.find(glyph) {
            Some(id) => match self.font_set.char_strings[self.id].get(id) {
                Some(char_string) => {
                    Program::new(char_string, &self.font_set.global_subroutines,
                                 &self.font_set.local_subroutines[self.id])
                },
                _ => reject!(),
            },
            _ => return Ok(None),
        };

        let mut builder = Builder::new();

        let mut clear = false;
        while let Some((operator, operands)) = try!(program.next()) {
            macro_rules! get(($index:expr) => (operands[$index]));
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
                        builder.cubic_to(
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
                        builder.cubic_to(
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
                            builder.cubic_to(
                                (get!(j + 0),         0.0),
                                (get!(j + 1), get!(j + 2)),
                                (       last, get!(j + 3)),
                            );
                        } else {
                            builder.cubic_to(
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
                            builder.cubic_to(
                                (get!(j + 0),         0.0),
                                (get!(j + 1), get!(j + 2)),
                                (       last, get!(j + 3)),
                            );
                        } else {
                            builder.cubic_to(
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
                        builder.cubic_to(
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
                        builder.cubic_to(
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
                    builder.cubic_to(
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

impl TrueType {
    #[inline]
    pub fn new(glyph_data: Rc<GlyphData>, mapping: Rc<Mapping>) -> Self {
        TrueType { glyph_data: glyph_data, mapping: mapping }
    }
}

impl Case for TrueType {
    fn draw(&self, glyph: char) -> Result<Option<Glyph>> {
        use truetype::glyph_data::{Description, Simple};

        macro_rules! reject(() => (panic!("found a malformed glyph")));

        let glyph = match self.mapping.find(glyph) {
            Some(id) => match self.glyph_data.get(id) {
                Some(&Some(ref glyph)) => glyph,
                Some(&None) => return Ok(Some(Default::default())),
                _ => reject!(),
            },
            _ => return Ok(None),
        };

        let mut builder = Builder::new();

        match &glyph.description {
            &Description::Simple(Simple { ref end_points, ref flags, ref x, ref y, .. }) => {
                macro_rules! is_control(($i:expr) => (flags[$i] & 0b1 == 0));
                macro_rules! read(($i:expr) => ((x[$i] as f32, y[$i] as f32)));
                let point_count = flags.len();
                if point_count == 0 || point_count != x.len() || point_count != y.len() {
                    reject!();
                }
                let mut cursor = 0;
                for &end in end_points {
                    let end = end as usize;
                    if end >= point_count || is_control!(cursor) {
                        reject!();
                    }
                    let first = read!(cursor);
                    builder.move_to(first);
                    let mut control: Option<(f32, f32)> = None;
                    for cursor in (cursor + 1)..(end + 1) {
                        let current = read!(cursor);
                        if is_control!(cursor) {
                            match &mut control {
                                &mut Some(ref mut control) => {
                                    let x = (control.0 + current.0) / 2.0;
                                    let y = (control.1 + current.1) / 2.0;
                                    builder.quadratic_to(*control, Some((x, y)));
                                    *control = current;
                                },
                                control @ &mut None => {
                                    *control = Some(current);
                                },
                            }
                        } else {
                            match control.take() {
                                Some(control) => builder.quadratic_to(control, Some(current)),
                                _ => builder.line_to(current),
                            }
                        }
                    }
                    if let Some(control) = control.take() {
                        builder.quadratic_to(control, None);
                    }
                    cursor = end + 1;
                }
            },
            &Description::Compound(..) => unimplemented!(),
        }

        Ok(Some(builder.into()))
    }
}
