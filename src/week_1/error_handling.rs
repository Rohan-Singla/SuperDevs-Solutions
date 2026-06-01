pub fn parse_number(s: &str) -> Result<i32, String> {
    let parse = s.parse::<i32>();

    parse.map_err(|_| "invalid number".to_string())
}

pub fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    let aparse = a.parse::<i32>().map_err(|_| "invalid number".to_string())?;

    let bparse = b.parse::<i32>().map_err(|_| "invalid number".to_string())?;

    Ok(aparse + bparse)
}

pub fn parse_csv_nums(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err("String is empty".to_string());
    }

    let mut sum = 0;

    for i in s.split(",") {
        let n = i.trim().parse::<i32>().map_err(|_| "Invalid")?;

        sum += n;
    }

    return Ok(sum);
}
