extern crate bit_vec;

use bit_vec::BitVec;

fn sieve_of_eratosthenes(bound : usize) -> Vec<usize> {
    let mut is_prime = BitVec::from_elem(bound, true);

    // 0 and 1 are not prime
    is_prime.set(0, false);
    is_prime.set(1, false);

    for i in 2..1 + (bound as f64).sqrt() as usize {
        if is_prime[i] {
            for j in i.. {
                if i * j >= bound {
                    break;
                }
                is_prime.set(i * j, false);
            }
        }
    }
    let mut primes = Vec::new();
    for (i, elem) in is_prime.iter().enumerate() {
        if elem {
            primes.push(i);
        }
    }
    primes
}

fn find_greatest_prime_factor(number : usize) -> Option<usize> {
    // Greatest factor can be half the numebr
    for prime in sieve_of_eratosthenes(number / 2).into_iter().rev() {
        if number % prime == 0 {
            return Some(prime);
        }
    }
    if number >= 2 {
        Some(number)
    } else {
        None
    }
}

fn main() {
    let greatest_prime_factor = find_greatest_prime_factor(49870000);
    println!("{}", greatest_prime_factor.unwrap());
}
