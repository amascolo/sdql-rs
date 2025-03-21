use sdql_runtime::{date, Date, OrderedFloat};

pub mod parallel;
pub mod sequential;

pub type TypeQ6 = OrderedFloat<f64>;

const _19940101: Date = date!(19940101);
const _19950101: Date = date!(19950101);
