fn find_greatest_prime_factor(number : usize) -> Option<usize> {
    if number < 2 {
        None
    } else {
        let mut factors = Vec::new();
        let mut num = number;
        let mut divisor = 2;
        while num > 1 {
            while num % divisor == 0 {
                factors.push(divisor);
                num /= divisor;
            }
            divisor += 1;
            if divisor * divisor > num {
                factors.push(num);
                break;
            }
        }
        factors.pop()
    }
}

fn main() {
    let greatest_prime_factor = find_greatest_prime_factor(600851475143);
    println!("{}", greatest_prime_factor.unwrap());
}
