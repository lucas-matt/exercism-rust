pub fn nth(n: u32) -> u32 {
    let max = (n+1) as usize;
    let primes = &mut vec![2];
    for num in 2.. {
        if &primes.len() > &max {
            break;
        }
        let nonprime = primes.into_iter().any(|prime| num % *prime == 0);
        if !nonprime {
            primes.push(num);
        }
    }
    primes[n as usize]
}
