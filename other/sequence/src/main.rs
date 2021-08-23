use itertools;
use std::io;

fn main() {
    print_seq(get_num());
}

fn get_num() -> u8 {
    println!("Insert the desired length (2 or greater):");
    let mut num = String::new();
    loop {
        io::stdin()
            .read_line(&mut num)
            .expect("Could not parse your input");
        match num.trim().parse() {
            Ok(n) => {
                if n >= 2 {
                    break n;
                } else {
                    println!("Number should be 2 or greater");
                    continue;
                }
            }
            Err(_) => {
                println!("Bad input");
                continue;
            }
        }
    }
}

// Prints sequence like this:
// 1
// 4 6
// 9 11 13
// 17 19 21 23
fn print_seq(length: u8) {
    if length < 2 {
        panic!("Invalid length");
    }
    // This fibonacci sequence skips 0, 1 and 1
    let (mut penultimate_fib, mut last_fib) = (1, 2);
    let mut t;

    // start of the line of the printed sequence
    let mut seq_start: u32 = 1;
    let mut line: Vec<u32> = Vec::new();

    println!("{}", seq_start);

    for line_num in 2..length + 1 {
        // progress fibonacci seq
        t = penultimate_fib;
        penultimate_fib = last_fib;
        last_fib = last_fib + t;
        // calc new line start
        seq_start += last_fib;

        line.clear();
        for line_el_index in 0..line_num {
            line.push(seq_start + line_el_index as u32 * 2);
        }

        println!("{}", itertools::join(line.iter(), " "));
    }
}
