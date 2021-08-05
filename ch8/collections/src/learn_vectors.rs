enum Cell {
	Index(i32),
	Value(f64),
	Comment(String),
}

pub fn main() {
	let mut v: Vec<i32> = Vec::new();
	v.push(5);
	v.push(6);
	println!("Vector = {:?}", v);
	println!("Len = {:?}", v.len());
	println!(
		"Last el = {}",
		match v.pop() {
			Some(val) => val,
			None => 0,
		}
	);
	println!("Vector after pop = {:?}", v);
	v.extend([1, 2, 3].iter().copied());
	println!("Vector after extend = {:?}", v);

	let v = vec![1, 2, 3, 4];
	let third: &i32 = &v[2];
	println!("Third element = {}", third);

	// we get Option<&T>
	match v.get(2) {
		Some(third) => println!("Third element = {}", third),
		None => println!("There is no third element"),
	}

	// rust panicked
	// let does_not_exist = &v[100];
	// no panic
	let does_not_exist = v.get(100);

	let mut v = vec![1, 2, 3, 4, 5];
	let first = &v[0];
	// Can't do that as we have an immutable reference (first)
	// v.push(6);
	println!("First element = {}", first);

	let mut v = vec![1, 2, 3, 4, 5];
	let mut first = &mut v[0];
	// Can't do that as we borrow v as mut twice
	// v.push(6);
	println!("First element = {}", first);

	let mut v = vec![10, 9, 8];
	for i in &mut v {
		*i += 1;
	}
	println!("Vector = {:?}", v);
	let cut = &v[..2];
	println!("Cut = {:?}", cut);

	let row = vec![
		Cell::Index(3),
		Cell::Value(100.0),
		Cell::Comment(String::from("Max value")),
	];
}
