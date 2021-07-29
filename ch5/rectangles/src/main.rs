#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect = {:?}", rect);
    println!("Area of rect = {}", rect.area());

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 25,
        height: 55,
    };

    println!("");
    println!("rect2 = {:?}", rect2);
    println!(
        "Can rect hold rect2? - {}",
        if rect.can_hold(&rect2) { "Yes" } else { "No" }
    );
    println!("rect3 = {:?}", rect3);
    println!(
        "Can rect hold rect3? - {}",
        if rect.can_hold(&rect3) { "Yes" } else { "No" }
    );

    println!("");
    let square = Rectangle::square(10);
    println!("square = {:?}", square);
    println!("Area of a square = {}", square.area());
}
