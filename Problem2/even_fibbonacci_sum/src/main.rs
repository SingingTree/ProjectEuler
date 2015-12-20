fn even_fibonacci_sum(bound : usize) -> usize {
	let mut last_num_1 = 0;
	let mut last_num_2 = 1;
	let mut evens_sum = 0;
	let mut next_num;
	loop {
		next_num = last_num_1 + last_num_2;
		if next_num > bound {
			break
		} else {
			if next_num % 2 == 0 {
				evens_sum += next_num
			}
		}
		last_num_1 = last_num_2;
		last_num_2 = next_num;
	}
	evens_sum
}

fn main() {
	let sum = even_fibonacci_sum(4000000);
    println!("{}", sum);
}
