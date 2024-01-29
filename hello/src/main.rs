use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing arguments"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
    check_autocomplete();
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


// This is a comment and this function is not called yet
#[warn(dead_code)]
fn check_autocomplete() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
}

// Create a function that finds the average of three numbers
#[warn(dead_code)]
fn _average(x: i32, y: i32, z: i32) -> f64 {
    let sum = x + y + z;
    let avg = sum as f64 / 3.0;
    // do a loop that prints {},{},{} x ,y z
    println!("The average of {}, {}, {} is {}", x, y, z, avg);
    // test comment
    avg
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}