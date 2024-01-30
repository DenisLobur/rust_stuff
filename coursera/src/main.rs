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

    let mut aPerson = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    };

    println!("Person's first name is {} and their age is {}", aPerson.first_name, aPerson.age);
    aPerson.set_age(30);
    println!("Person's first name is {} and their age is {}", aPerson.first_name, aPerson.age);

    let point = Point(10, 20, 30);
    println!("The point first coordinate is: {},", point.0);
}
