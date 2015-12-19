use std::collections::HashSet;

fn find_mutiple_sum(divisors : &[i32], multiple_bound: i32) -> i32 {
	let mut multiples = HashSet::new();
	for divisor in divisors {
		let mut i = 1;
		while divisor * i < multiple_bound {
			multiples.insert(divisor * i);
			i += 1;
		}	
	}
	multiples.into_iter().fold(0, |acc, num| acc + num)
}

fn main() {
	let sum = find_mutiple_sum(&[3, 5], 1000);
    println!("{}", sum);
}
