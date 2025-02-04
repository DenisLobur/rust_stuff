use std::{error, fs};
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {

    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())
    //let mut error_logs = vec![];

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(&text);

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Written to errors.txt successful"),
    //             Err(e) => println!("Writting to errors.txt failed: {}", e),
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(&text);
    // fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write to errors.txt");

    //println!("{:#?}", error_logs);

    // match divide(5.0, 0.0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match validate_email(String::from("email")) {
    // Ok(..) => println!("Valid email"),
    // Err(e) => println!("Error: {}", e),
    // }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Invalid email"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
