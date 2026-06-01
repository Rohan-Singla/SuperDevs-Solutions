pub fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}

pub fn swap(a: i32, b: i32) -> (i32, i32) {
    let mut pair = (a, b);

    (pair.1, pair.0) = pair;

    return pair;
}

pub fn first_last(number: &[i32]) -> (i32, i32) {
    let array_length = number.len();

    let first = number[0];

    let last = number[array_length - 1];

    // let slice = &number[0..array_length];

    return (first, last);
}
