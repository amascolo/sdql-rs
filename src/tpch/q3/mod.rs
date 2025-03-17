use crate::utils::round;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use sdql_runtime::{Bool, Date, HashMap, Record, date};

pub mod parallel;
pub mod sequential;

pub type TypeQ3 = HashMap<Record<(i32, Date, i32, OrderedFloat<f64>)>, Bool>;

const _19950315: Date = date!(19950315);

pub fn format_q3_result(result: &TypeQ3) -> String {
    result
        .iter()
        .map(|(key, val)| {
            format!(
                "<{},{},{},{}>:{}\n",
                key.0,
                key.1,
                key.2,
                round(key.3, 4),
                (**val) as i32,
            )
        })
        .sorted()
        .collect()
}
