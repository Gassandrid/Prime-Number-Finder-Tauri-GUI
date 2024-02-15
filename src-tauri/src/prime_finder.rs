pub fn eratos(limit: u32) -> Vec<u32> {
    let mut primes = vec![true; (limit + 1) as usize];
    primes[0] = false;
    primes[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if primes[p as usize] {
            let mut i = p * p;
            while i <= limit {
                primes[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut result = Vec::new();
    for (num, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            result.push(num as u32);
        }
    }

    result
}
