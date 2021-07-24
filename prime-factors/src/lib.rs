pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec!();
    let mut factor = 2;
    loop {
        if n == 1 || factor > n {
            break
        }
        match n % factor {
            0 => {
                factors.push(factor);
                n = n / factor
            },
            _ => factor += 1
        }
    }
    factors
}
