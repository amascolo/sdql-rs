/// Converts a date string (e.g. "1995-03-15") into an integer (e.g. 19950315).
pub fn date_to_numeric(date_str: &str) -> i32 {
    date_str.replace("-", "").parse().unwrap()
}

/// Converts an integer date (e.g. 19950315) back into a formatted string "1995-03-15".
pub fn format_date(numeric: i32) -> String {
    let year = numeric / 10000;
    let month = (numeric / 100) % 100;
    let day = numeric % 100;
    format!("{year:04}-{month:02}-{day:02}")
}

pub fn round(f: f64, n: u32) -> f64 {
    let factor = 10_f64.powi(n as i32);
    (f * factor).round() / factor
}
