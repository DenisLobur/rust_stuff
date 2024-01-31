use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
            age: 0,
        }
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

struct Point(i32, i32, i32);

enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum Disksize {
    MB(u32),
    GB(u32),
    TB(u32),
}

enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let file = File::open("not_existing_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => println!("File not found!"),
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };

    let mut a_person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    };

    println!(
        "Person's first name is {} and their age is {}",
        a_person.first_name, a_person.age
    );
    a_person.set_age(30);
    println!(
        "Person's first name is {} and their age is {}",
        a_person.first_name, a_person.age
    );

    let point = Point(10, 20, 30);
    println!("The point first coordinate is: {},", point.0);

    // Strings and slices

    let sentence = "The quick brown fox jumps over the lazy dog.";
    let description = format!("The short story of:\n {}", sentence);
    println!("{}", description);

    // iterate ofer the characters of the string
    for c in sentence.chars() {
        println!("{}", c);
    }

    // split and collect into a vector
    let words = sentence.split(" ").collect::<Vec<&str>>();
    let words2: Vec<&str> = sentence.split_whitespace().collect();
    println!("words: {:?}", words);
    println!("words2: {:?}", words2);

    // Reverse a string
    let reversed = sentence.chars().rev().collect::<String>();
    println!("Reversed: {}", reversed);

    // vectors
    let vec = vec![1, 2, 3, 4, 5];
    let last_value = vec.last().unwrap();
    println!("Last value: {:?}", last_value);
    match vec.first() {
        Some(first_value) => println!("First value: {:?}", first_value),
        None => println!("Vector is empty!"),
    }

    // enums

    let disk = DiskType::SSD;
    match disk {
        DiskType::SSD => println!("Solid State Drive"),
        DiskType::HDD => println!("Hard Disk Drive"),
    }

    let disk_size = Disksize::TB(1);
    println!("Disk size: {:?}", disk_size);

    // enums + vectors + map fun
    let shapes = vec![Shape::Circle(10.0), Shape::Square(20.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
        })
        .sum();

    println!("Total area: {}", total_area);
}
