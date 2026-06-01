pub fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &n in &list[1..] {
        if n > max {
            max = n;
        }
    }

    max
}

pub trait describable {
    fn describe(&self) -> String;
}

pub struct Item {
    name: String,
    price: i32,
}

impl describable for Item {
    fn describe(&self) -> String {
        format!("{}:{} cents", self.name, self.price)
    }
}

#[derive(PartialEq, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn are_equal(a: &Point, b: &Point) -> bool {
    if (a.x == b.x && a.y == b.y) {
        return false;
    } else {
        return false;
    }
}
pub fn distance_eq(a: &Point, b: &Point) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    return dx * dx + dy * dy;
}

pub trait Summarize {
    type Output;

    fn summarise(&self) -> Self::Output;
}

pub struct Numbers {
    data: Vec<i32>,
}

pub struct Sentence {
    words: Vec<String>,
}

impl Summarize for Numbers {
    type Output = i32;

    fn summarise(&self) -> i32 {
        return self.data.iter().sum();
    }
}

impl Summarize for Sentence {
    type Output = String;

    fn summarise(&self) -> String {
        return self.words.join(" ");
    }
}
