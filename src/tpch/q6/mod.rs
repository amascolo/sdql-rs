use crate::utils::round;
use ordered_float::OrderedFloat;
use sdql_runtime::{date, Date};

pub mod parallel;
pub mod sequential;

pub type TypeQ6 = OrderedFloat<f64>;

const _19940101: Date = date!(19940101);
const _19950101: Date = date!(19950101);

pub fn format_q6_result(result: &TypeQ6) -> String {
    format!("{}\n", round(*result, 4))
}
