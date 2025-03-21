use sdql_runtime::{date, Bool, Date, HashMap, OrderedFloat, Record};

pub mod parallel;
pub mod sequential;

pub type TypeQ3 = HashMap<Record<(i32, Date, i32, OrderedFloat<f64>)>, Bool>;

const _19950315: Date = date!(19950315);
