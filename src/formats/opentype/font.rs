use std::cell::RefCell;
use std::io::Result;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use opentype::truetype::Tag;

use crate::formats::opentype::cache::{Cache, Reference};
use crate::formats::opentype::{axes, characters, features, metrics, names, palettes, tables};

/// A font.
pub struct Font<T> {
    cache: Reference<Cache<T>>,
    index: (bool, usize),
}

/// A disposition.
pub enum Disposition {
    Retain,
    Update,
}

impl<T: typeface::tape::Read> crate::font::Case for Font<T> {
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

    fn glyph(&mut self, character: char) -> Result<Option<crate::Glyph>> {
        let mut cache = self.cache.borrow_mut();
        let mapping = cache.mapping()?.clone();
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

pub fn read<T: typeface::tape::Read>(
    tape: Reference<T>,
    backend: opentype::Font,
) -> Result<Vec<Font<T>>> {
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
    T: typeface::tape::Read + 'static,
    U: typeface::tape::Read + typeface::tape::Write,
    F: Fn(&Tag) -> Disposition,
{
    if !font.index.0 {
        raise!("writing PostScript fonts is not supported yet");
    }
    let offset = tape.position()?;
    let cache = font.cache.borrow_mut();
    let mut other = cache.tape.borrow_mut();
    let mut offsets = cache.backend.offsets.clone();
    tape.give(&offsets)?;
    let size = tape.position()? - offset;
    pad(tape, size as usize)?;
    for record in offsets.records.iter_mut() {
        let offset = tape.position()?;
        let checksum = match dispose(&record.tag) {
            Disposition::Retain => {
                other.jump(record.offset as u64)?;
                let mut table = std::io::Read::take(other.by_ref(), record.size as u64);
                std::io::copy(&mut table, tape)?;
                false
            }
            Disposition::Update => {
                match &*record.tag {
                    b"name" => match cache.names.as_ref() {
                        Some(table) => tape.give(table.borrow().deref())?,
                        _ => raise!("found no update for {:?}", record.tag),
                    },
                    _ => raise!("updating {:?} is not supported yet", record.tag),
                }
                true
            }
        };
        record.offset = offset as _;
        record.size = (tape.position()? - offset) as _;
        pad(tape, record.size as usize)?;
        if checksum {
            record.checksum = record.checksum(tape)?;
        }
    }
    tape.jump(offset)?;
    tape.give(&offsets)?;
    Ok(())
}

fn pad<T: typeface::tape::Write>(tape: &mut T, size: usize) -> Result<()> {
    match size % 4 {
        1 => tape.give_bytes(&[0, 0, 0])?,
        2 => tape.give_bytes(&[0, 0])?,
        3 => tape.give_bytes(&[0])?,
        _ => {}
    }
    Ok(())
}
