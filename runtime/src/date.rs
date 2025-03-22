use crate::date;
use approx::AbsDiffEq;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::AddAssign;
use time::format_description::well_known::Iso8601;
use time::{format_description, Month};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Date(pub time::Date);

impl Date {
    pub const fn new(date: time::Date) -> Self {
        Date(date)
    }

    pub const fn from(year: i32, month: u32, day: u8) -> Self {
        let month: Month = month_from_int(month);
        match time::Date::from_calendar_date(year, month, day) {
            Ok(date) => Self::new(date),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = format_description::parse("[year][month][day]").unwrap();
        let yyyymmdd = self.0.format(&description).unwrap();
        write!(f, "date({yyyymmdd})")
    }
}
// TODO Debug only needed until we have Record::Display
impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl std::str::FromStr for Date {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        time::Date::parse(s, &Iso8601::DEFAULT).map(Date)
    }
}

// we need this as Month::try_from isn't const
pub const fn month_from_int(m: u32) -> Month {
    match m {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => unreachable!(),
    }
}

#[macro_export]
macro_rules! date {
    ($yyyymmdd:literal) => {{
        const YEAR: i32 = ($yyyymmdd / 10000) as i32;
        const MONTH: u32 = ($yyyymmdd / 100) % 100;
        const DAY: u8 = ($yyyymmdd % 100) as u8;
        $crate::Date::from(YEAR, MONTH, DAY)
    }};
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

impl AbsDiffEq for Date {
    type Epsilon = ();

    fn default_epsilon() -> Self::Epsilon {
        ()
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.eq(other)
    }
}
