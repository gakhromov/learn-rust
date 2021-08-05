use std::collections::HashMap;

pub fn main() {
	let mut scores: HashMap<String, i32> = HashMap::new();
	let team = String::from("yellow");
	let score = 50;
	scores.insert(String::from("blue"), 10);
	scores.insert(team, score);
	println!("{:?}", scores);
	// Can't do this as team is moved
	// println!("{}", team);
	// Can do this as i32 has Copy trait
	println!("{}", score);

	let teams = vec![String::from("blue"), String::from("yellow")];
	let init_scores = vec![10, 50];
	let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

	println!("{:?}", scores);

	let team_name = String::from("blue");
	// Can panic
	let score = scores[&team_name];
	// Or this (no panic)
	let score = scores.get(&team_name);
	match score {
		Some(val) => println!("{} score = {}", team_name, val),
		None => (),
	}

	for (key, val) in &scores {
		println!("Team {} has {} points", key, val);
	}

	let mut scores: HashMap<&str, i32> = HashMap::new();
	scores.insert("blue", 10);
	println!("Initial = {:?}", scores);

	scores.insert("blue", 25);
	println!("After insert = {:?}", scores);

	scores.entry("yellow").or_insert(50);
	scores.entry("blue").or_insert(50);
	println!("After entry or insert = {:?}", scores);

	count_words("hello world wonderful world");
}

fn count_words(text: &str) {
	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{:?}", map);
}
