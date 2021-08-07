const VOCALS: [&str; 15] = [
	"e", "u", "i", "o", "a", "у", "е", "ы", "а", "о", "э", "ё", "я", "и", "ю",
];

pub fn main() {
	let inp = String::from("first");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("apple");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("первый");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("яблоко");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("я");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
	let inp = String::from("s");
	println!("\"{}\" in pig-latin is \"{}\"", inp, pig_latin(&inp));
}

fn pig_latin(s: &String) -> String {
	let mut chars = s.char_indices();

	// get the first char
	if let Some((_, char)) = chars.next() {
		// check for vocals
		if VOCALS.contains(&&char.to_lowercase().to_string()[..]) {
			return format!("{}-hay", s);
		}
		// get next char index to cut the string
		if let Some((index, _)) = chars.next() {
			return format!("{}-{}ay", &s[index..], char);
		}
	}
	String::from("")
}
