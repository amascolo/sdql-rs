use ordered_float::{OrderedFloat, PrimitiveFloat};
use std::fmt;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Real<T: PrimitiveFloat>(OrderedFloat<T>);

impl<T> fmt::Display for Real<T>
where
    T: PrimitiveFloat + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.0.0)
    }
}
