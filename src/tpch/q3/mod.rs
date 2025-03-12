use crate::runtime::{Date, Record};
use crate::utils::round;
use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;

pub mod parallel;
pub mod sequential;

pub type TypeQ3 = HashMap<Record<(i32, Date, i32, OrderedFloat<f64>)>, bool>;

const _19950315: Date = crate::date!(19950315);

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
                *val as i32,
            )
        })
        .sorted()
        .collect()
}
