fn find_primes(num_primes : usize) -> Vec<usize> {
    // Sieve of Eratosthenes style approach
    let mut primes = Vec::new();
    if num_primes == 0 {
        return primes;
    }
    primes.push(2);

    let mut i = 3;
    while primes.len() < num_primes {
        let mut is_prime = true;
        for prime in primes.iter() {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 1;
    }
    primes
}

fn main() {
    println!("{}", find_primes(10001).pop().unwrap());
}
