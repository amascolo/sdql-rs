use crate::runtime::Date;
use crate::utils::round;

pub mod parallel;
pub mod sequential;

type TypeQ6 = f64;

const _19940101: Date = crate::date!(19940101);
const _19950101: Date = crate::date!(19950101);

pub fn format_q6_result(result: &TypeQ6) -> String {
    format!("{}\n", round(*result, 4))
}
