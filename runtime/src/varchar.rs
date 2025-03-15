use arrayvec::{ArrayString, CapacityError};
use derive_more::Display;
use std::ops::{AddAssign, Deref};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{_0}")]
pub struct VarChar<const CAP: usize>(ArrayString<CAP>);

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
