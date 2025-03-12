use derive_more::Display;
use std::ops::{AddAssign, Deref};

pub const TRUE: Bool = Bool(true);
pub const FALSE: Bool = Bool(false);

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{_0}")]
pub struct Bool(bool);

impl AddAssign for Bool {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 || rhs.0);
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
