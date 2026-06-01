pub fn sum_to(n: i32) -> i32 {
    let mut total = 0;

    for i in 1..=n {
        total += i
    }

    return total;
}

pub fn transform(s: &str) -> i32 {
    let trimmed: &str = s.trim();

    let trimmed: i32 = trimmed.parse().unwrap();

    let trimmed: i32 = trimmed * trimmed;

    return trimmed;
}
