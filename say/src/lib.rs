pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let num = n.to_string();
    let num = format!("{}{}", "0".repeat(3 - (num.len() % 3)), num);
    let segments = num.chars().collect::<Vec<char>>().windows(3).step_by(3)
        .map(|c| c.iter().collect::<String>())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    segments.iter().rev().zip(vec!("", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion")).rev()
        .map(|(n, scale)| segment(*n, scale))
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}

fn segment(n:u64, scale:&str) -> String {
    let mut result = String::new();
    let (hundreds, tens, units) = htu(n);

    // append hundreds
    if let Some(hundreds) = base(hundreds) {
        result.push_str(format!("{} hundred ", hundreds).as_str())
    }

    // append tens
    let tens = base(tens);
    if let Some(tens) = tens {
        result.push_str(tens);
    }

    // append units
    if let Some(units) = base(units) {
        let join = if tens.is_none() { "" } else { "-" };
        result.push_str(format!("{}{}", join, units).as_str());
    }

    // include scale (millions / billions)
    if !result.is_empty() {
        result.push_str(format!(" {}", scale).as_str());
    }
    result.trim().to_string()
}

fn htu(mut n: u64) -> (u64, u64, u64) {
    let h = n / 100;
    n -= h * 100;
    if base(n).is_some() {
        (h, n, 0)
    } else {
        let t = (n / 10) * 10;
        (h, t, n - t)
    }
}

fn base<'a>(n: u64) -> Option<&'a str> {
    let result = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => return None
    };
    Some(result)
}
