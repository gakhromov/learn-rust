use std::io;
use std::collections::HashMap;

enum Command {
	Add,
	Print,
	Exit,
	Unknown,
}

pub fn main() {
	let mut company: HashMap<String, Vec<String>> = HashMap::new();

	println!("Enter \"exit\" to exit.");
	loop {
		let mut inp = String::new();
		println!("Enter command: ");
		io::stdin().read_line(&mut inp)
			.expect("Error reading line");
		
		let (command, args) = parse_command(&inp);
		match command {
			Command::Add => add_to_dep(&mut company, &args),
			Command::Print => print_company(&company),
			Command::Exit => break,
			Command::Unknown => println!("Unknown command"),
		}
	}
}

fn parse_command(s: &String) -> (Command, Vec<String>) {
	let words: Vec<String> = s.split_ascii_whitespace().map(str::to_string).collect();
	match words.first() {
		Some(word) => {
			match &word.to_lowercase()[..] {
				"add" => (Command::Add, (&words[1..]).to_vec()),
				"print" => (Command::Print, vec![]),
				"exit" => (Command::Exit, vec![]),
				_ => (Command::Unknown, vec![]),
			}
		},
		None => (Command::Unknown, vec![]),
	}
}

fn print_company(company: &HashMap<String, Vec<String>>) {
	for (dep_name, employees) in company.iter() {
		println!("Department \"{}\":", dep_name);
		for employee in employees {
			println!("\t{}", employee);
		}
	}
}

fn split_name_and_dep_name(args: &Vec<String>) -> Option<(String, String)> {
	for (i, arg) in args.iter().enumerate() {
		if arg.eq_ignore_ascii_case("to") {
			let name = (&args[..i].to_vec().join(" ")).to_string();
			// no panic if nothing after "to", will be just and empty vec
			let dep_name = (&args[i+1..].to_vec().join(" ")).to_string();
			return Some((name, dep_name));
		}
	}
	println!("Error: no \"to\" in command");
	None
}

fn add_to_dep(company: &mut HashMap<String, Vec<String>>, args: &Vec<String>) {
	if let Some((name, dep_name)) = split_name_and_dep_name(args) {
		let employees = company.entry(dep_name.to_lowercase()).or_insert(vec![]);
		employees.push(name);
	}
}
