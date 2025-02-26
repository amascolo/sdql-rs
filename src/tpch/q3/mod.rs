use crate::tpch::runtime::month_from_int;
use crate::utils::round;
use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use time::{Date, Month};

pub mod parallel;
pub mod sequential;

type TypeQ3 = HashMap<(i32, Date, i32, OrderedFloat<f64>), i32>;

const _19950315: Date = crate::const_date!(19950315);

pub fn format_q3_result(result: &TypeQ3) -> String {
    result
        .iter()
        .map(|(key, val)| {
            format!(
                "<{},{},{},{}>:{}\n",
                key.0,
                key.1,
                key.2,
                round(key.3.into_inner(), 4),
                val
            )
        })
        .sorted()
        .collect()
}
