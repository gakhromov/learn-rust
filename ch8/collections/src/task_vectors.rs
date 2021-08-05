use std::collections::HashMap;

pub fn main() {
	let a = [1, 5, 3, 4, 6, 8, 5, 3, 5];

	let mut v: Vec<u8> = Vec::with_capacity(9);
	v.extend(a.iter());

	println!("a = {:?}", a);
	println!("mean = {}", mean(&v));

	v.sort();
	println!("a sorted = {:?}", v);
	println!("median = {}", median_for_sorted(&v));
	println!("mode = {}", mode(&v));
}

fn mean(v: &Vec<u8>) -> f64 {
	let mut sum = 0;

	for el in v {
		sum += *el;
	}

	sum as f64 / v.len() as f64
}

fn is_odd(n: u8) -> bool {
	let last_digit = n % 10;
	if [1, 3, 5, 7, 9].contains(&last_digit) {
		return true;
	} else {
		return false;
	}
}

fn median_for_sorted(v: &Vec<u8>) -> f64 {
	if is_odd(v.len() as u8) {
		let middle = v.len() as u8 / 2;
		v[middle as usize] as f64
	} else {
		let middle1 = v.len() as u8 / 2;
		let middle2 = middle1 - 1;
		(v[middle1 as usize] as f64 + v[middle2 as usize] as f64) / 2.0
	}
}

fn mode(v: &Vec<u8>) -> u8 {
	let mut map: HashMap<&u8, u8> = HashMap::new();

	for el in v {
		let cnt = map.entry(el).or_insert(0);
		*cnt += 1;
	}

	let mut max_key: u8 = 0;
	let mut max_val = 0;

	for (key, val) in map {
		if val > max_val {
			max_val = val;
			max_key = *key;
		}
	}
	max_key
}
