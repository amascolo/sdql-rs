use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Display, Sub, SubAssign, Sum};
use ordered_float::{OrderedFloat, PrimitiveFloat};
use std::hash::{Hash, Hasher};
use std::ops::Mul;
use std::str::FromStr;

#[repr(transparent)]
#[derive(
    Clone, Copy, Debug, Display, Default, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, Sum,
)]
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

impl<T> AbsDiffEq for Real<T>
where
    T: PrimitiveFloat + AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.0.abs_diff_eq(&other.0.0, epsilon)
    }
}
