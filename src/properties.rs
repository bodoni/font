//! Properties.

/// Properties.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties {
    /// The outline format.
    pub outline: Outline,
    /// The style.
    pub style: Style,
    /// The variations.
    pub variations: Option<Variations>,
}

/// An outline format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outline {
    PostScript,
    TrueType,
}

/// A style.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    /// An indicator for the bold style.
    pub bold: bool,
    /// An indicator for the italic style.
    pub italic: bool,
}

/// Variations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variations;
