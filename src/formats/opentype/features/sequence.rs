use std::cmp::Ordering;
use std::collections::BTreeSet;

/// A sequence.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sequence {
    Single(Position),
    Range((char, char)),
    List(Vec<Position>),
}

/// A position.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Position {
    Single(char),
    Range((char, char)),
    Set(BTreeSet<Position>),
}

macro_rules! equal(
    ($ordering:expr, $fallback:ident $(,)?) => (match $ordering {
        Ordering::Equal => Ordering::$fallback,
        value => value,
    });
);

impl std::cmp::Ord for Sequence {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Single(one), Self::Single(other)) => one.cmp(other),
            (Self::Single(one), Self::Range(other)) => one.cmp(&Position::Range(*other)),
            (Self::Single(one), Self::List(other)) => equal!(
                other
                    .first()
                    .map(|other| one.cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::Range(one), Self::Single(other)) => Position::Range(*one).cmp(other),
            (Self::Range(one), Self::Range(other)) => one.cmp(other),
            (Self::Range((one, _)), Self::List(other)) => equal!(
                other
                    .first()
                    .map(|other| Position::Single(*one).cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::List(one), Self::Single(other)) => equal!(
                one.first()
                    .map(|one| one.cmp(other))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::List(one), Self::Range((other, _))) => equal!(
                one.first()
                    .map(|one| one.cmp(&Position::Single(*other)))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::List(one), Self::List(other)) => one.cmp(other),
        }
    }
}

impl std::cmp::PartialOrd for Sequence {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Single(one), Self::Single(other)) => one.cmp(other),
            (Self::Single(one), Self::Range((other, _))) => equal!(one.cmp(other), Less),
            (Self::Single(one), Self::Set(other)) => equal!(
                other
                    .first()
                    .map(|other| Position::Single(*one).cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::Range((one, _)), Self::Single(other)) => equal!(one.cmp(other), Greater),
            (Self::Range(one), Self::Range(other)) => one.cmp(other),
            (Self::Range((one, _)), Self::Set(other)) => equal!(
                other
                    .first()
                    .map(|other| Position::Single(*one).cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::Set(one), Self::Single(other)) => equal!(
                one.first()
                    .map(|one| one.cmp(&Position::Single(*other)))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::Set(one), Self::Range((other, _))) => equal!(
                one.first()
                    .map(|one| one.cmp(&Position::Single(*other)))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::Set(one), Self::Set(other)) => one.cmp(other),
        }
    }
}

impl std::cmp::PartialOrd for Position {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
