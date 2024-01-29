use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("not_existing_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => panic!("File not found!"),
                _ => panic!("Problem opening the file: {:?}", error)
                
            }
        }
    };
}
