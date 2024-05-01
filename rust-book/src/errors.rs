use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn creating_file_with_result() {
    let greeting_file_result = File::open("some/file/path");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test_file.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("Problem creating file {:?}", e)
            }
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };
}

pub fn creating_file_with_closure() {
    let file = File::open("").unwrap();
    let file2 = File::open("").expect("some error here");

    File::open("some/file/path").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error)
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    //let guess = Guess::new(10);
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, Got {} instead", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}