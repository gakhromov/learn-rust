use std::io;

fn main() {
    println!("Input temperature (F or C)");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Could not read the line");

    let temp: f64 = temp.trim().parse()
        .expect("Could not parse the number");

    println!("{} C = {} F", temp, celc_to_fahr(temp));
    println!("{} F = {} C", temp, fahr_to_celc(temp));
    
}

fn celc_to_fahr(temp: f64) -> f64 {
    (temp * 9.0/5.0) + 32.0
}

fn fahr_to_celc(temp: f64) -> f64 {
    (temp - 32.0) * 5.0/9.0
}
