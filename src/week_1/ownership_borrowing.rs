pub fn join_string(a: String, b: String) -> String {
    let mya = a;
    let myb = b;

    return mya + " " + &myb;
}

pub fn count_chars(s: &str) -> usize {
    let chars = s.chars().count();

    return chars;
}

pub fn double_all(nums: &mut Vec<i32>) -> &Vec<i32> {
    for n in nums.iter_mut() {
        *n *= 2;
    }

    return nums;
}

pub fn first_word(s: &str) -> String {
    let word = s.split_whitespace().next().unwrap_or("").to_string();

    return word;
}
