use std::collections::HashMap;

pub fn most_frequent(s: &str) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    for word in s.split_whitespace() {
        let lower = word.to_lowercase();
        if !counts.contains_key(&lower) {
            order.push(lower.clone());
        }
        *counts.entry(lower).or_insert(0) += 1;
    }

    order.into_iter().max_by_key(|word| counts[word]).unwrap()
}

pub fn diagonal_sum(input: &str) -> i32 {
    let matrix: Vec<Vec<i32>> = input
        .split(';')
        .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let n = matrix.len();

    let mut sum = 0;
    for i in 0..n {
        sum += matrix[i][i];
        sum += matrix[i][n - 1 - i];
    }

    if n % 2 == 1 {
        sum -= matrix[n / 2][n / 2];
    }

    sum
}

pub fn evaluate(expr: &str) -> Result<i32, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("invalid expression".to_string());
    }

    let a = parts[0]
        .parse::<i32>()
        .map_err(|_| "invalid expression".to_string())?;
    let op = parts[1];
    let b = parts[2]
        .parse::<i32>()
        .map_err(|_| "invalid expression".to_string())?;

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0 {
                Err("division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("unknown operator".to_string()),
    }
}
