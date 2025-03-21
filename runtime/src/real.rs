use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use ordered_float::{OrderedFloat, PrimitiveFloat};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Mul;

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, Sum)]
pub struct Real<T>(OrderedFloat<T>)
where
    T: PrimitiveFloat,
    OrderedFloat<T>: Eq + Ord + Hash;

// TODO why deriving doesn't work
impl<T: PrimitiveFloat> Eq for Real<T> {}
impl<T: PrimitiveFloat> Ord for Real<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T: PrimitiveFloat> Hash for Real<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<T: PrimitiveFloat> Mul for Real<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Real(self.0 * rhs.0)
    }
}

impl<T> Real<T>
where
    T: PrimitiveFloat,
{
    pub fn new(float: T) -> Self {
        Self(OrderedFloat(float))
    }
}

use std::str::FromStr;

impl<T> fmt::Display for Real<T>
where
    T: PrimitiveFloat + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.0.0)
    }
}
// TODO Debug only needed until we have Record::Display
impl<T> fmt::Debug for Real<T>
where
    T: PrimitiveFloat + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<T> FromStr for Real<T>
where
    T: PrimitiveFloat + FromStr,
    OrderedFloat<T>: FromStr,
{
    type Err = <OrderedFloat<T> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OrderedFloat::<T>::from_str(s).map(Self)
    }
}
