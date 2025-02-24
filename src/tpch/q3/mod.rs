use crate::utils::print_date;
use hashbrown::HashMap;
use ordered_float::OrderedFloat;

pub mod parallel;
pub mod sequential;

type TypeQ3 = HashMap<(i32, i32, i32, OrderedFloat<f64>), i32>;

pub fn print_q3_result(result: TypeQ3) {
    for (key, val) in result.iter() {
        println!(
            "<{}, {}, {}, {:.4}>:{}",
            key.0,
            print_date(key.1),
            key.2,
            key.3,
            val
        );
    }
}
