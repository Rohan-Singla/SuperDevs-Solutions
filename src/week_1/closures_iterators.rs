pub fn double_all(nums: &[i32]) -> Vec<i32> {
    let double = nums.iter().map(|&x| x * 2).collect();

    return double;
}

pub fn sum_of_squares(nums: &[i32]) -> i32 {
    let mut sum = 0;

    let square: Vec<i32> = nums.iter().map(|&x| x * x).collect();

    for i in square {
        sum += i;
    }

    return sum;
}

pub fn evens_only(nums: &[i32]) -> Vec<i32> {
    let mut even: Vec<i32> = vec![];

    for i in nums {
        if i % 2 == 0 {
            even.push(*i);
        }
    }

    return even;
}

pub struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let current = self.n;
            self.n -= 1;
            Some(current)
        } else {
            None
        }
    }
}
