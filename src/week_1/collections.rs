use std::collections::HashSet;

pub fn sum_vec(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    return sum;
}

pub fn count_vowels(s: &str) -> usize {
    let vowels = "aeiouAEIOU";

    let count = s.chars().filter(|c: &char| vowels.contains(*c)).count();

    return count;
}

pub fn unique_word_count(s: &str) -> usize {
    s.split_whitespace().collect::<HashSet<&str>>().len()
}
