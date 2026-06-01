pub fn classify(n: i32) -> &'static str {
    if (n > 0) {
        return "positive";
    } else if (n < 0) {
        return "negative";
    } else {
        return "zero";
    }
}

pub fn longest<'a>(x: &'a str, b: &'a str) -> &'a str {
    let xlength = x.len();

    let blenth = b.len();

    if (xlength > blenth) {
        return return x;
    } else if (xlength == blenth) {
        return return x;
    } else {
        return b;
    }
}

pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    let result = s.starts_with(prefix);

    if (result) {
        return &s[prefix.len()..];
    } else {
        return s;
    }
}
