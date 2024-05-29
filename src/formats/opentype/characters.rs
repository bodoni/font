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
            } else {
                values.push(Character::Range((start, end)));
            }
        }
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::Character;

    macro_rules! ok(($result:expr) => ($result.unwrap()));

    macro_rules! characters(
        ($($variant:ident($($argument:tt)*),)*) => (
            vec![$(characters!(@one $variant, $($argument)*),)*]
        );
        (@one Scalar, $value:literal) => (
            Character::Scalar(ok!(char::try_from($value as u32)))
        );
        (@one Range, ($start:literal, $end:literal)) => (
            Character::Range((
                ok!(char::try_from($start as u32)),
                ok!(char::try_from($end as u32)),
            ))
        );
    );

    #[test]
    fn compress() {
        let ranges = vec![
            (0, 0),
            (13, 13),
            (32, 47),
            (48, 57),
            (58, 59),
            (63, 64),
            (65, 90),
            (91, 96),
            (97, 122),
            (123, 126),
            (160, 165),
            (168, 169),
            (171, 174),
            (176, 176),
            (180, 183),
            (187, 187),
            (191, 191),
            (192, 207),
            (209, 214),
            (216, 221),
            (223, 239),
            (241, 246),
            (248, 253),
            (255, 255),
            (305, 305),
            (338, 339),
            (352, 353),
            (376, 376),
            (381, 382),
            (402, 402),
            (710, 711),
            (728, 730),
            (732, 733),
            (916, 916),
            (937, 937),
            (956, 956),
            (960, 960),
            (8211, 8212),
            (8216, 8218),
            (8220, 8222),
            (8226, 8226),
            (8230, 8230),
            (8240, 8240),
            (8249, 8250),
            (8260, 8260),
            (8364, 8364),
            (8482, 8482),
            (8486, 8486),
            (8706, 8706),
            (8710, 8710),
            (8719, 8719),
            (8721, 8722),
            (8730, 8730),
            (8734, 8734),
            (8747, 8747),
            (8776, 8776),
            (8800, 8800),
            (9674, 9674),
            (61698, 61698),
            (64257, 64258),
        ];
        assert_eq!(
            ok!(super::compress(ranges)),
            characters![
                Scalar(0),
                Scalar(13),
                Range((32, 59)),
                Range((63, 126)),
                Range((160, 165)),
                Range((168, 169)),
                Range((171, 174)),
                Scalar(176),
                Range((180, 183)),
                Scalar(187),
                Range((191, 207)),
                Range((209, 214)),
                Range((216, 221)),
                Range((223, 239)),
                Range((241, 246)),
                Range((248, 253)),
                Scalar(255),
                Scalar(305),
                Range((338, 339)),
                Range((352, 353)),
                Scalar(376),
                Range((381, 382)),
                Scalar(402),
                Range((710, 711)),
                Range((728, 730)),
                Range((732, 733)),
                Scalar(916),
                Scalar(937),
                Scalar(956),
                Scalar(960),
                Range((8211, 8212)),
                Range((8216, 8218)),
                Range((8220, 8222)),
                Scalar(8226),
                Scalar(8230),
                Scalar(8240),
                Range((8249, 8250)),
                Scalar(8260),
                Scalar(8364),
                Scalar(8482),
                Scalar(8486),
                Scalar(8706),
                Scalar(8710),
                Scalar(8719),
                Range((8721, 8722)),
                Scalar(8730),
                Scalar(8734),
                Scalar(8747),
                Scalar(8776),
                Scalar(8800),
                Scalar(9674),
                Scalar(61698),
                Range((64257, 64258)),
            ],
        );
    }
}
