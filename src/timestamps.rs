/// Timestamps.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Timestamps {
    /// The time of creation in seconds relative to 1904-01-01T00:00:00+00:00.
    pub creation: i64,
    /// The time of modification in seconds relative to 1904-01-01T00:00:00+00:00.
    pub modification: i64,
}
