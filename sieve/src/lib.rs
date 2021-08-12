pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 {
        return vec!()
    }
    let mut primes:Vec<u64> = vec!();
    let mut range: Vec<u64> = (2..upper_bound+1).collect();
    while !range.is_empty() {
        let &prime = range.iter().next().unwrap();
        primes.push(prime);
        range = range.into_iter().filter(|&n| n % prime != 0).collect();
    }
    primes
}
