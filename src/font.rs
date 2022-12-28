use crate::case::Case;

/// A font.
pub struct Font(Box<dyn Case>);

dereference! { Font::0 => Box<dyn Case> }

#[inline]
pub fn new(case: Box<dyn Case>) -> Font {
    Font(case)
}
