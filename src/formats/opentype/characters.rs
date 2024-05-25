//! Unicode characters.

use std::io::Result;

use opentype::truetype::tables::character_mapping::Encoding;

use crate::formats::opentype::cache::Cache;

/// A Unicode character.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Character {
    Scalar(char),
    Range((char, char)),
}

/// Unicode characters.
pub type Characters = Vec<Character>;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Vec<Character>> {
    for encoding in cache.character_mapping()?.borrow().encodings.iter() {
        let ranges = match encoding {
            Encoding::Format0(encoding) => encoding.characters(),
            Encoding::Format4(encoding) => encoding.characters(),
            Encoding::Format6(encoding) => encoding.characters(),
            Encoding::Format12(encoding) => encoding.characters(),
            _ => continue,
        };
        return compress(ranges);
    }
    raise!("found no known character-to-glyph encoding")
}

fn compress(ranges: Vec<(u32, u32)>) -> Result<Vec<Character>> {
    let mut values = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let (Some(start), Some(end)) = (char::from_u32(range.0), char::from_u32(range.1)) {
            if let Some(value) = values.last_mut() {
                let (first, last) = match value {
                    Character::Scalar(first) => (*first, *first),
                    Character::Range((first, last)) => (*first, *last),
                };
                if last as usize + 1 == start as usize {
                    *value = Character::Range((first, end));
                    continue;
                }
            }
            if start == end {
                values.push(Character::Scalar(start));
            } else if start as usize + 1 == end as usize {
                values.push(Character::Scalar(start));
                values.push(Character::Scalar(end));
            } else {
                values.push(Character::Range((start, end)));
            }
        }
    }
    Ok(values)
}
