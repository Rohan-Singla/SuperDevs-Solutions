#[macro_export]
macro_rules! square {
    ($x:expr) => { $x * $x };
}

pub fn compute(n: i32) -> i32 {
    return square!(n)
}

#[macro_export]
macro_rules! convert {
    (celsius_to_f, $temp:expr) => { $temp * 9 / 5 + 32 };
    (f_to_celsius, $temp:expr) => { ($temp - 32) * 5 / 9 };
}

pub fn temp_test(c: i32) -> i32 {
    convert!(celsius_to_f, c)
}

#[macro_export]
macro_rules! sum {
    () => { 0 };
    ( $( $x:expr ),* ) => { 0 $( + $x )* };
}

pub fn total(a: i32, b: i32, c: i32) -> i32 {
    sum!(a, b, c)
}