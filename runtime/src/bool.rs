use approx::AbsDiffEq;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, Deref};
use std::str::{FromStr, ParseBoolError};

pub const FALSE: Bool = Bool(false);
pub const TRUE: Bool = Bool(true);

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
)]
#[display("{_0}")]
pub struct Bool(bool);

// note: += (therefore the newtype Bool) isn't needed for TPC-H queries since they all use unique()
impl AddAssign for Bool {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 || rhs.0);
    }
}

impl FromStr for Bool {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AbsDiffEq for Bool {
    type Epsilon = ();

    fn default_epsilon() -> Self::Epsilon {
        ()
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.eq(other)
    }
}
