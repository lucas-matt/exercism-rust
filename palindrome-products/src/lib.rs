#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd)]
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
    let palindomes:Vec<Palindrome> = (min..=max).flat_map(|x| {
        (min..=max)
            .filter(move |y| y <= &x)
            .map(move |y| Palindrome::new(y, x))
    })
        .filter(|p| is_palindrome(&p.value()))
        .collect();
    let smallest = palindomes.iter().min();
    let largest = palindomes.iter().max();
    match (smallest, largest) {
        (Some(&smallest), Some(&largest)) => Some((smallest, largest)),
        _ => None
    }
}

fn is_palindrome(p: &u64) -> bool {
    p.to_string().chars().collect::<String>() == p.to_string().chars().rev().collect::<String>()
}
