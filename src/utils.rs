use ordered_float::OrderedFloat;

pub fn round(f: OrderedFloat<f64>, n: u32) -> OrderedFloat<f64> {
    let factor = 10_f64.powi(n as i32);
    OrderedFloat((f * factor).round() / factor)
}
