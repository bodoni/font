//! Properties.

/// Properties.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties {
    /// The outline format.
    pub outline: Outline,
}

/// An outline format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outline {
    PostScript,
    TrueType,
}
