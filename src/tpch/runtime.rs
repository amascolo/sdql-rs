use crate::runtime::Date;
use std::marker::PhantomData;
use time::Month;

// so IDE doesn't remove import
const _: PhantomData<Date> = PhantomData;

#[allow(dead_code)]
// note: Month::try_from isn't const
pub(crate) const fn month_from_int(m: u32) -> Month {
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
macro_rules! const_date {
    ($yymmdd:literal) => {{
        const YEAR: i32 = ($yymmdd / 10000) as i32;
        const MONTH: time::Month = crate::tpch::runtime::month_from_int(($yymmdd / 100) % 100);
        const DAY: u8 = ($yymmdd % 100) as u8;

        match time::Date::from_calendar_date(YEAR, MONTH, DAY) {
            Ok(date) => Date(date),
            _ => unreachable!(),
        }
    }};
}
