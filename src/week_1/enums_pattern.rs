pub fn coin_value(coin: &str) -> i32 {
    match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    }
}

pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if (b == 0) {
        return None;
    } else {
        return Some(a / b);
    }
}

pub fn get_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "data_types",
        _ => "F",
    }
}
