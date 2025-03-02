use crate::const_date;
use derive_more::Display;
use std::ops::AddAssign;
use time::error;
use time::parsing::Parsable;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{_0:?}")]
pub struct Date(pub time::Date);

impl Default for Date {
    fn default() -> Self {
        const_date!(00010101) // dummy
    }
}

impl AddAssign for Date {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs; // overwrite dummy
    }
}

impl Date {
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        Ok(Self(time::Date::parse(input, description)?))
    }
}
