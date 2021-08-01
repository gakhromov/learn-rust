use std::{
    cmp::Ordering,
    io::{self, Write},
};
// Same as
// use std::io;
// use std::io::Write;
// use std::cmp::Ordering;

use std::collections::*;

mod front_of_house; // import mod from front_of_house.rs

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("banana"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");
    println!("Toast = {}", meal.toast);

    // Does not compile:
    // meal.seasonal_fruit = String::from("pineapple");
    // println!("Seasonal fruit = {}", meal.seasonal_fruit);

    let order = back_of_house::Appetizer::Salad;
}

// Absolute
use crate::front_of_house::hosting;
// or
// use front_of_house::hosting;
// Relative
// use self::front_of_house::hosting;

fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

pub use front_of_house::atmosphere;
// now others can use
// restaurant::atmosphere::control_light();
// instead of
// restaurant::front_of_house::atmosphere::control_light();

mod other_mod {
    pub mod some_mod;
}

fn test_not_using_other_mod_rs() {
    other_mod::some_mod::some_fn();
}
