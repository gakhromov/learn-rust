use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    const FILENAME: &str = "hello.txt";

    let f = match File::open(FILENAME) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILENAME) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            _ => panic!("Error opening file: {:?}", error),
        },
    };

    // or

    let f = File::open(FILENAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILENAME).unwrap_or_else(|error| {
                panic!("Problem when creating file: {:?}", error);
            })
        } else {
            panic!("Problem when opening file: {:?}", error);
        }
    });

    // try to read file or panic
    let f = File::open(FILENAME).unwrap();

    // try to read file or panic with your own message
    let f = File::open(FILENAME).expect(&format!("Can't open \"{}\"", FILENAME)[..]);

    let username = read_username_from_file(FILENAME).unwrap();
    println!("Found name \"{}\"", username);

    let username = better_read_username_from_file(FILENAME).unwrap();
    println!("Found name \"{}\"", username);

    let username = super_short_read_username_from_file(FILENAME).unwrap();
    println!("Found name \"{}\"", username);
}

fn read_username_from_file(filename: &str) -> Result<String, Error> {
    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e), // propogate error
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(String::from(s.trim_end())),
        Err(e) => Err(e),
    }
}

fn better_read_username_from_file(filename: &str) -> Result<String, Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s.trim_end().to_string())
}

fn super_short_read_username_from_file(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
}
