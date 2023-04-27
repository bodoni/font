/// Properties.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Properties {
    /// The vendor identifier.
    pub vendor_id: String,
    /// An indicator for the bold style.
    pub bold: bool,
    /// An indicator for the italic style.
    pub italic: bool,
    /// An indicator for cubic Bézier curves.
    pub cubic: bool,
    /// An indicator for font variations.
    pub variable: bool,
}
