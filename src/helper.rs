pub fn format_percent(percent: f32) -> (String, bool) {
    let is_positive = percent.is_sign_positive();
    let prefix = if is_positive { "+" } else { "" };

    (format!("{}{}%", prefix, percent), is_positive)
}

pub fn format_bignum(number: f32) -> String {
    let mut number = number as i64;
    let suffixes = ["K", "M", "B", "T"];
    let mut suffix = "";
    if number > 1000 {
        for s in &suffixes {
            suffix = s;
            if number < 1_000_000 {
                break;
            }
            number /= 1_000;
        }
    }
    format!("{:.3}{}", (number as f64) / 1000., suffix)
}
