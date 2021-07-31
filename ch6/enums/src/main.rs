enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and so on
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Quarter from state {:?}", state);
                25
            }
        }
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 50 };

    let coin = Coin::Quarter(UsState::Alabama);
    println!("{}", coin.value_in_cents());

    let some_u8_value = Some(0u8);
    // this is
    if let Some(3) = some_u8_value {
        println!("Three");
    }
    // the same as
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }
}
