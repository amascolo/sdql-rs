use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Display, Sub, SubAssign, Sum};
use std::hash::{Hash, Hasher};
use std::ops::Mul;
use std::str::FromStr;

#[repr(transparent)]
#[derive(
    Clone, Copy, Debug, Display, Default, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, Sum,
)]
pub struct Int(i32);

// TODO why deriving doesn't work
impl Eq for Int {}
impl Ord for Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl Hash for Int {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl Mul for Int {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}

impl Int {
    fn new(int: i32) -> Self {
        Self(int)
    }
}

impl FromStr for Int {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(Int::new)
    }
}

impl AbsDiffEq for Int {
    type Epsilon = ();

    fn default_epsilon() -> Self::Epsilon {
        ()
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.0.eq(&other.0)
    }
}
