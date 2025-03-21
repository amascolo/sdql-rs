use approx::AbsDiffEq;
use derive_more::Display;
use std::ops::{AddAssign, Deref};

pub const FALSE: Bool = Bool(false);
pub const TRUE: Bool = Bool(true);

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

impl AbsDiffEq for Bool {
    type Epsilon = ();

    fn default_epsilon() -> Self::Epsilon {
        ()
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.eq(other)
    }
}
