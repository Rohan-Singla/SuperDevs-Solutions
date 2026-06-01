pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}

pub fn abs_value(n: i32) -> i32 {
    return if (n < 0) { -n } else { n };
}

pub fn factorial(n: u64) -> u64 {
    let mut myfactorial = 1;

    for i in 2..=n {
        myfactorial *= i;
    }

    return myfactorial;
}

pub fn fizbuzz(n: i32) -> String {
    if (n % 3 == 0 && n % 5 == 0) {
        return "FizzBuzz".to_string();
    } else if (n % 3 == 0 && n % 5 != 0) {
        return "Fizz".to_string();
    } else if (n % 3 != 0 && n % 5 == 0) {
        return "Buzz".to_string();
    } else {
        return n.to_string();
    }
}
