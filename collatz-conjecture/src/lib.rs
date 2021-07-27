pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps = 0;
    while n > 1 {
        n = match n {
            n if n % 2 == 0 => n / 2,
            _ => 3 * n + 1
        };
        steps += 1
    }
    match n {
        0 => None,
        _ => Some(steps)
    }
}
