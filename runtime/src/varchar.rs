use approx::AbsDiffEq;
use arrayvec::{ArrayString, CapacityError};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{AddAssign, Deref};
use std::str::FromStr;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct VarChar<const CAP: usize>(ArrayString<CAP>);

impl<const CAP: usize> fmt::Display for VarChar<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
// TODO Debug only needed until we have Record::Display
impl<const CAP: usize> fmt::Debug for VarChar<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<const CAP: usize> VarChar<CAP> {
    pub fn from(s: &str) -> Result<Self, CapacityError<&str>> {
        ArrayString::from(s).map(VarChar)
    }
}

impl<const CAP: usize> FromStr for VarChar<CAP> {
    type Err = CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ArrayString::from_str(s).map(VarChar)
    }
}

impl<const CAP: usize> AddAssign for VarChar<CAP> {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs; // overwrite
    }
}

impl<const CAP: usize> Deref for VarChar<CAP> {
    type Target = ArrayString<CAP>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const CAP: usize> AbsDiffEq for VarChar<CAP> {
    type Epsilon = ();

    fn default_epsilon() -> Self::Epsilon {
        ()
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.eq(other)
    }
}
