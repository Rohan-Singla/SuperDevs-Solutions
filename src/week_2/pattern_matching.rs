pub fn classify(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        n @ 1..=10 => format!("small: {}", n),
        n @ -10..=-1 => format!("neg small: {}", n),
        n => format!("big: {}", n),
    }
}

pub fn parse_command(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.as_slice() {

        ["quit"] => "Goodbye".to_string(),
        
        ["echo", rest @ ..] => rest.join(" "),
        ["add", x, y] => {
            let sum: i32 = x.parse::<i32>().unwrap_or(0) + y.parse::<i32>().unwrap_or(0);
            sum.to_string()
        }
        ["repeat", n, msg] => {
            let count = n.parse::<usize>().unwrap_or(0);
            vec![*msg; count].join(" ")
        }
        _ => "Unknown".to_string(),
    }
}