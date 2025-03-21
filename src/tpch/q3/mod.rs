use itertools::Itertools;
use sdql_runtime::{date, Bool, Date, HashMap, OrderedFloat, Record};

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
                key.3,
                (**val) as i32,
            )
        })
        .sorted()
        .collect()
}
