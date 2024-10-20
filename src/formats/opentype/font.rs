use std::cell::RefCell;
use std::io::Result;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use opentype::truetype::tables::FontHeader;
use opentype::truetype::Tag;

use crate::formats::opentype::cache::{Cache, Reference};
use crate::formats::opentype::{
    axes, characters, features, metrics, names, palettes, tables, timestamps,
};

/// A font.
pub struct Font<T> {
    cache: Reference<Cache<T>>,
    index: (bool, usize),
}

/// A disposition.
#[derive(Eq, PartialEq)]
pub enum Disposition {
    Retain,
    Update,
}

impl<T: crate::Read> crate::font::Case for Font<T> {
    #[inline]
    fn axes(&mut self) -> Result<crate::Axes> {
        axes::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn characters(&mut self) -> Result<crate::Characters> {
        characters::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn features(&mut self) -> Result<crate::Features> {
        features::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn metrics(&mut self) -> Result<crate::Metrics> {
        metrics::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn names(&mut self) -> Result<crate::Names> {
        names::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn palettes(&mut self) -> Result<crate::Palettes> {
        palettes::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn tables(&mut self) -> Result<crate::Tables> {
        tables::read(&mut self.cache.borrow_mut())
    }

    #[inline]
    fn timestamps(&mut self) -> Result<crate::Timestamps> {
        timestamps::read(&mut self.cache.borrow_mut())
    }

    fn glyph(&mut self, character: char) -> Result<Option<crate::Glyph>> {
        let mut cache = self.cache.borrow_mut();
        let mapping = cache.forward_mapping()?.clone();
        let metrics = cache.metrics()?.clone();
        match self.index {
            (true, _) => {
                super::truetype::draw(&cache.glyph_data()?.borrow(), &mapping, &metrics, character)
            }
            (false, id) => super::postscript::draw(
                &cache.font_set()?.borrow(),
                &mapping,
                &metrics,
                id,
                character,
            ),
        }
    }
}

pub fn read<T: crate::Read>(tape: Reference<T>, backend: opentype::Font) -> Result<Vec<Font<T>>> {
    use opentype::postscript::compact1::FontSet;
    use opentype::truetype::tables::GlyphData;

    let truetype = backend.exists::<GlyphData>();
    let postscript = {
        let mut tape = tape.borrow_mut();
        let tape = tape.deref_mut();
        backend
            .position::<_, FontSet>(tape)?
            .map(|_| FontSet::count(tape))
            .transpose()?
            .unwrap_or(0)
    };
    let cache = Rc::new(RefCell::new(Cache::new(tape, backend)));
    let mut fonts = vec![];
    if truetype {
        fonts.push(Font {
            cache: cache.clone(),
            index: (true, 0),
        });
    }
    for id in 0..postscript {
        fonts.push(Font {
            cache: cache.clone(),
            index: (false, id),
        });
    }
    Ok(fonts)
}

/// Write a font.
pub fn write<T, U, F>(font: Font<T>, tape: &mut U, dispose: F) -> Result<()>
where
    T: crate::Read + 'static,
    U: crate::Read + crate::Write,
    F: Fn(&Tag) -> Disposition,
{
    let mut cache = font.cache.borrow_mut();

    let offsets_position = tape.position()?;
    let mut offsets = cache.backend.offsets.clone();

    let mut font_header_position = None;
    let mut font_header = *cache.font_header()?.borrow();
    font_header.checksum_adjustment = 0;

    tape.give(&offsets)?;
    let size = tape.position()? - offsets_position;
    pad(tape, size as usize)?;

    let mut other = cache.tape.borrow_mut();
    for record in offsets.records.iter_mut() {
        let position = tape.position()?;
        let disposition = if record.tag == b"head" {
            font_header_position = Some(position);
            Disposition::Update
        } else {
            dispose(&record.tag)
        };
        match disposition {
            Disposition::Retain => {
                other.jump(record.offset as u64)?;
                copy(other.deref_mut(), tape, record.size as u64)?;
            }
            Disposition::Update => match &*record.tag {
                b"head" => tape.give(&font_header)?,
                b"name" => match cache.names.as_ref() {
                    Some(table) => tape.give(table.borrow().deref())?,
                    _ => raise!("found no update for {:?}", record.tag),
                },
                _ => raise!("updating {:?} is not supported yet", record.tag),
            },
        }
        record.offset = position as _;
        record.size = (tape.position()? - position) as _;
        pad(tape, record.size as usize)?;
        if disposition == Disposition::Update {
            record.checksum = record.checksum(tape)?;
        }
    }

    tape.jump(offsets_position)?;
    tape.give(&offsets)?;

    let font_header_position = match font_header_position {
        Some(value) => value,
        _ => raise!("found no font header"),
    };

    tape.jump(offsets_position)?;
    let checksum = FontHeader::checksum(tape)?;
    font_header.checksum_adjustment = FontHeader::CHECKSUM_ADJUSTMENT.wrapping_sub(checksum);
    tape.jump(font_header_position)?;
    tape.give(&font_header)?;

    Ok(())
}

fn copy<T, U>(source: &mut T, destination: &mut U, size: u64) -> Result<()>
where
    T: crate::Read + 'static,
    U: crate::Read + crate::Write,
{
    let mut source = std::io::Read::take(source.by_ref(), size);
    std::io::copy(&mut source, destination)?;
    Ok(())
}

fn pad<T: crate::Write>(tape: &mut T, size: usize) -> Result<()> {
    match size % 4 {
        1 => tape.give_bytes(&[0, 0, 0])?,
        2 => tape.give_bytes(&[0, 0])?,
        3 => tape.give_bytes(&[0])?,
        _ => {}
    }
    Ok(())
}
