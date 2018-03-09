pub fn format_percent(percent: &str) -> (String, bool) {
    let percent: f32 = percent.parse().unwrap();
    let is_positive = percent.is_sign_positive();
    let prefix = if is_positive { "+" } else { "" };

    (format!("{}{} %", prefix, percent), is_positive)
}

pub fn divmod(n: i32, d: i32) -> (i32, i32) {
    (n / d, n % d)
}

pub fn format_price(nums: &str) -> String {
    let number: f32 = nums.parse().unwrap();
    let (dollars, cents) = divmod((number * 100.) as i32, 100);
    let mut s = String::new();
    for (i, char) in dollars.to_string().chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 {
            s.insert(0, ',');
        }
        s.insert(0, char);
    }
    format!("${}.{}", s, cents)
}

pub fn format_bignum(nums: &str) -> String {
    let number: f64 = nums.parse().unwrap();
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
