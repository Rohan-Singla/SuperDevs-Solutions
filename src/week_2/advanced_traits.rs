use std::{fmt, ops::Add};

pub struct CommaSeparated(pub Vec<i32>);

impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

pub fn format_list(nums: Vec<i32>) -> String {
    CommaSeparated(nums).to_string()
}

trait Summary {
    type Output;

    fn summarize(&self) -> Self::Output;
}

struct Numbers {
    data: Vec<i32>,
}
struct Words {
    data: Vec<String>,
}

impl Summary for Numbers {
    type Output = i32;
    fn summarize(&self) -> Self::Output {
        let mut sum = 0;

        for i in &self.data {
            sum += i;
        }

        return sum;
    }
}

impl Summary for Words {
    type Output = String;

    fn summarize(&self) -> Self::Output {
        return self.data.join(" ");
    }
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

pub fn add_vecs(a: Vec2, b: Vec2) -> String {
    format!("{}", a + b)
}
