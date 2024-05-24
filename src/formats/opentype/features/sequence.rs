use std::collections::BTreeSet;

/// A sequence.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sequence {
    Single(Position),
    Range(char, char),
    List(Vec<Position>),
}

/// A position.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Position {
    Single(char),
    Range(char, char),
    Set(BTreeSet<Position>),
}

impl Sequence {
    fn first(&self) -> Option<char> {
        match self {
            Self::Single(value) => value.first(),
            Self::Range(value, _) => Some(*value),
            Self::List(value) => value.first().and_then(Position::first),
        }
    }
}

impl std::cmp::Ord for Sequence {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first().cmp(&other.first())
    }
}

impl std::cmp::PartialOrd for Sequence {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    fn first(&self) -> Option<char> {
        match self {
            Self::Single(value) => Some(*value),
            Self::Range(value, _) => Some(*value),
            Self::Set(value) => value.first().and_then(Position::first),
        }
    }
}

impl std::cmp::Ord for Position {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first().cmp(&other.first())
    }
}

impl std::cmp::PartialOrd for Position {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
