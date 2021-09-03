#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    a:u64, b:u64
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            a, b
        }
    }

    pub fn value(&self) -> u64 {
        self.a * self.b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.a = a;
        self.b = b;
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let max_len = max.to_string().len();
    let palindromes:Vec<u64> = palindromes(max_len * max_len)
        .iter()
        .map(|d| d.parse::<u64>().unwrap())
        .filter(|&d| min*min <= d && d <= max*max)
        .collect();
    None
}

const digits:&[char;10] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn palindromes(size: usize) -> Vec<String> {
    if size == 1 {
        let mut single:Vec<String> = digits.to_vec().iter().map(|d| d.to_string()).collect();
        let double:Vec<String> = single.iter().map(|d| format!("{}{}", d, d)).collect();
        single.extend(double);
        return single;
    }
    let previous = palindromes(size - 1);
    let mut palindromes = previous.clone();
    for palindrome in previous.into_iter() {
        for digit in digits {
            palindromes.push(format!("{}{}{}", digit, palindrome, digit))
        }
    }
    palindromes
        .into_iter()
        .filter(|p| !p.starts_with('0'))
        .collect()
}