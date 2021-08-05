pub fn main() {
	let mut s = String::new();

	let mut s = "Initial content".to_string();
	// Equivalent to
	let mut s = String::from("Initial content");
	s = s + ", more content";
	s = format!("{}, even more content", s);
	s.push_str(", fin");
	s.push('!');
	println!("{}", s);

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;
	// Can't do that, as s1 is moved
	// println!("S1 = {}", s1);
	println!("S2 = {}", s2);
	println!("S3 = {}", s3);

	// len is the number of bytes used for encoding
	println!("Len of \"Hello\" = {}", String::from("Hello").len());
	println!("Len of \"Привет\" = {}", String::from("Привет").len());
	let mut s = String::from("Hello");
	s.push('П');
	println!("Len of \"{}\" = {}", s, s.len());

	let s = String::from("Привет");
	// Not allowed
	// let sub = &s[0];
	// Allowed
	let sub = &s[0..2];
	// Not allowed (panic)
	// let sub = &s[0..1];
	println!("{}", sub);

	println!("Chars in नमस्ते");
	for c in "नमस्ते".chars() {
		println!("{}", c);
	}

	println!("Bytes in नमस्ते");
	for c in "नमस्ते".bytes() {
		println!("{}", c);
	}
}
