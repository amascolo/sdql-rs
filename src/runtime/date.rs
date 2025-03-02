use crate::date;
use derive_more::Display;
use std::ops::AddAssign;
use time::Month;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{_0:?}")]
pub struct Date(time::Date);

impl Date {
    pub const fn new(date: time::Date) -> Self {
        Date(date)
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
    ($yymmdd:literal) => {{
        const YEAR: i32 = ($yymmdd / 10000) as i32;
        const MONTH: time::Month = crate::runtime::month_from_int(($yymmdd / 100) % 100);
        const DAY: u8 = ($yymmdd % 100) as u8;

        match time::Date::from_calendar_date(YEAR, MONTH, DAY) {
            Ok(date) => crate::runtime::Date::new(date),
            _ => unreachable!(),
        }
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
