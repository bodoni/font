/// Properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Properties {
    /// An indicator for cubic Bézier curves.
    pub cubic: bool,
    /// An indicator for the italic.
    pub italic: bool,
    /// An indicator for variations.
    pub variable: bool,
}
