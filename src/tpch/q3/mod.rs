use crate::utils::{format_date, round};
use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;

pub mod parallel;
pub mod sequential;

type TypeQ3 = HashMap<(i32, i32, i32, OrderedFloat<f64>), i32>;

pub fn format_q3_result(result: &TypeQ3) -> String {
    result
        .iter()
        .map(|(key, val)| {
            format!(
                "<{},{},{},{}>:{}\n",
                key.0,
                format_date(key.1),
                key.2,
                round(key.3.into_inner(), 4),
                val
            )
        })
        .sorted()
        .collect()
}
