mod pattern_match;
mod collections;
mod strings;
mod hashmaps;
mod errors;

use pattern_match::*;
use crate::collections::give_me_vector;
use crate::hashmaps::{check_hash_map, company_and_people, median_and_mode, pig_latin};
use crate::strings::using_strings;

fn main() {
    let user1 = User {
        username: "user1",
        email: String::from("someone"),
        sign_in_count: 0,
        active: false,
    };

    let user2 = User {
        username: "user2",
        ..user1
    };

    let color1 = Color(10, 20, 30);

    let any_struct = AnyStruct;

    println!("user1 name: {}", user1.username);
    println!("user2 name: {}", user2.username);
    println!("color1: {}, {}, {}", color1.0, color1.1, color1.2);

    let width = 30;
    let height = 20;

    let rect = Rectangle {
        width,
        height,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Area is equal: {}", area(width, height));
    println!("Area2 is equal: {}", area2((width, height)));
    println!("Area3 is equal: {}", area3(&rect));

    println!("Rect1 equals {:#?}", rect);
    println!("Area by method: {}", rect.area());

    println!("Can rect hold rect1? {}", rect.can_hold(&rect2));

    let sc = Rectangle::square(10);
    println!("Square: {:#?}", sc);

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let quarter = Coin::Quarter(UsState::Alabama);

    println!("Penny value: {:?}", value_in_cents(penny));
    println!("Nickel value: {:?}", value_in_cents(nickel));
    println!("Quarter value: {:?}", value_in_cents(quarter));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // give_me_vector();

    // using_strings();
    // check_hash_map();
    let mut numbers = vec![1, 2, 2, 3, 4, 4, 4, 5];
    let (median, mode) = median_and_mode(&mut numbers);
    println!("{median}, {mode}");

    let words = vec!["first", "apple"];
    for word in words {
        println!("{} -> {}", word, pig_latin(word));
    }

    company_and_people();
}

struct User<'a> {
    username: &'a str,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct AnyStruct;

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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
