
pub fn encode(source: &str) -> String {
    let mut stack:Vec<char> = Vec::new();
    let mut source = source.chars().peekable();
    let mut output:String = "".to_string();
    loop {
        let next = match source.next() {
            Some(char) => char,
            _ => {
                output = pack(output, stack);
                break;
            }
        };
        if !stack.is_empty() {
            if !stack.contains(&next) {
                output = pack(output, stack);
                stack = Vec::new();
            }
        }
        stack.push(next);
    };
    output
}

fn pack(output: String, stack:Vec<char>) -> String {
    if stack.is_empty() {
        return "".to_string();
    }
    let num = match stack.len() {
        1 => "".to_string(),
        n => n.to_string()
    };
    format!("{}{}{}", output, num, stack.get(0).unwrap())
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut source = source.chars().peekable();
    while source.peek().is_some() {
        let mut num:String = "".to_string();
        while source.peek().unwrap().is_digit(10) {
            num = format!("{}{}", num, source.next().unwrap());
        }
        let count = match num.parse::<usize>() {
            Ok(count) => count,
            _ => 1
        };
        decoded = decoded + source.next().unwrap().to_string().repeat(count).as_str();
    }
    decoded
}
