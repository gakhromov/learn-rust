use std::fmt::{Debug, Display};

pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {} ...)", self.summarize_author())
	}
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl NewsArticle {
	pub fn new() -> NewsArticle {
		NewsArticle {
			headline: String::from("headline"),
			location: String::from("location"),
			author: String::from("author"),
			content: String::from("content"),
		}
	}
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		self.author.clone()
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: String,
	pub retweet: String,
}

impl Tweet {
	pub fn new() -> Tweet {
		Tweet {
			username: String::from("user"),
			content: String::from("content"),
			reply: String::from("reply"),
			retweet: String::from("retweet"),
		}
	}
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
	fn summarize(&self) -> String {
		format!("{}: {}", self.summarize_author(), self.content)
	}
}

pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

// ===

pub fn notify_longer<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

// Different types that implement the same traits
// fn notify(item1: impl Summary + Display, item2: impl Summary + Display) {}

// Same types
// fn notify<T: Summary + Display>(item1: T, item2: T) {}

fn some_func<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
	0
}

// ===

fn some_fn<T, U>(t: T, u: U) -> i32
where
	T: Display + Clone,
	U: Clone + Debug,
{
	0
}

// Possible
fn return_summarizable() -> impl Summary {
	Tweet::new()
}

// Not possible (incompatible types)
// fn return_summarizable_2(is_news: bool) -> impl Summary {
// 	if is_news {
// 		NewsArticle::new()
// 	} else {
// 		Tweet::new()
// 	}
// }
