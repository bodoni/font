use std::cmp::Ordering;
use std::collections::BTreeSet;

/// A sample.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sample {
    Simple(Component),
    Alternate((char, usize)),
    Compound(Vec<BTreeSet<Component>>),
}

/// A component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Component {
    Scalar(char),
    Range((char, char)),
}

macro_rules! equal(
    ($ordering:expr, $fallback:ident $(,)?) => (match $ordering {
        Ordering::Equal => Ordering::$fallback,
        value => value,
    });
);

impl std::cmp::Ord for Sample {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Simple(one), Self::Simple(other)) => one.cmp(other),
            (Self::Simple(one), Self::Alternate((other, _))) => {
                equal!(one.cmp(&Component::Scalar(*other)), Less)
            }
            (Self::Simple(one), Self::Compound(other)) => equal!(
                other
                    .first()
                    .and_then(|other| other.first())
                    .map(|other| one.cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::Alternate((one, _)), Self::Simple(other)) => {
                equal!(Component::Scalar(*one).cmp(other), Greater)
            }
            (Self::Alternate(one), Self::Alternate(other)) => one.cmp(other),
            (Self::Alternate((one, _)), Self::Compound(other)) => equal!(
                other
                    .first()
                    .and_then(|other| other.first())
                    .map(|other| Component::Scalar(*one).cmp(other))
                    .unwrap_or(Ordering::Greater),
                Less,
            ),
            (Self::Compound(one), Self::Simple(other)) => equal!(
                one.first()
                    .and_then(|one| one.first())
                    .map(|one| one.cmp(other))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::Compound(one), Self::Alternate((other, _))) => equal!(
                one.first()
                    .and_then(|one| one.first())
                    .map(|one| one.cmp(&Component::Scalar(*other)))
                    .unwrap_or(Ordering::Less),
                Greater,
            ),
            (Self::Compound(one), Self::Compound(other)) => one.cmp(other),
        }
    }
}

impl std::cmp::PartialOrd for Sample {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Component {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Scalar(one), Self::Scalar(other)) => one.cmp(other),
            (Self::Scalar(one), Self::Range((other, _))) => equal!(one.cmp(other), Less),
            (Self::Range((one, _)), Self::Scalar(other)) => equal!(one.cmp(other), Greater),
            (Self::Range(one), Self::Range(other)) => one.cmp(other),
        }
    }
}

impl std::cmp::PartialOrd for Component {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
