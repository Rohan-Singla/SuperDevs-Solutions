pub fn double(x: i32) -> i32 {
    x * 2
}
pub fn increment(x: i32) -> i32 {
    x + 1
}

// says f is a function takes and return an i32
pub fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {

    // lets you call a function twice 
    f(f(x))
}


pub fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * n)
}

pub fn compose(f: Box<dyn Fn(i32) -> i32>, g: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| f(g(x)))
}