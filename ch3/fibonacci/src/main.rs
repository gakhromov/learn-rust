use std::io;

fn main() {
    println!("Enter a number");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read the line");
    let num: u8 = num.trim().parse().expect("Could not parse the number");

    if num < 1 {
        println!("Number should be >= 1");
        return;
    }
    let suffix = match num % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    println!("{}{} fibonacci number = {}", num, suffix, fibonacci(num));
}

fn fibonacci(num: u8) -> u64 {
    let (mut a, mut b) = (0, 1);
    let mut t;

    for _i in 1..num {
        t = a;
        a = a + b;
        b = t;
    }
    return a;
}
