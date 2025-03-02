use crate::date;
use derive_more::Display;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{_0:?}")]
pub struct Date(time::Date);

impl Date {
    pub const fn new(date: time::Date) -> Self {
        Date(date)
    }
}

impl Default for Date {
    fn default() -> Self {
        date!(00010101) // dummy
    }
}

impl AddAssign for Date {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs; // overwrite dummy
    }
}
