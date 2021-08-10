struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p = Point { x: 5.0, y: 10.0 };
    println!("|p-O| = {}", p.distance_from_origin());

    let number_vec = vec![3, 6, 8, 2, 10];
    println!("Max number in {:?} is {}", number_vec, largest(&number_vec));

    let number_lst = [13, 16, 28, 22, 10];
    println!("Max number in {:?} is {}", number_lst, largest(&number_lst));

    let char_lst = ['y', 'm', 'c', 'a', ' '];
    println!("Max char in {:?} is {}", char_lst, largest(&char_lst));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
