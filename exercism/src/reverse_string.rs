pub fn reverse_string(input: &str) -> String {
    let mut result: String = String::new();
    for char in input.chars() {
        result.insert(0, char);
    }

    println!("{}", result);
    result
}