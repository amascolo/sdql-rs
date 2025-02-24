use crate::utils::round;

pub mod parallel;
pub mod sequential;

type TypeQ6 = f64;

pub fn format_q6_result(result: &TypeQ6) -> String {
    format!("{}\n", round(*result, 4))
}
