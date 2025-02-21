// All primes from 2-N
pub fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime: Vec<bool> = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                println!("{j:?}");
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<usize> = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i) } else { None })
        .collect();

    primes
}
