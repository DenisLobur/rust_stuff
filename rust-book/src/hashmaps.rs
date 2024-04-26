use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn check_hash_map() {
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score_of_blue = score.get(&team_name).copied().unwrap_or(0);
    println!("Score of Blue: {:?}", score_of_blue);

    score.entry(String::from("Blue")).or_insert(50);
    println!("score: {:?}", score);

    for (key, value) in &score {
        println!("{key}: {value}");
    }

    calculate_entry();
}

fn calculate_entry() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

/**
 * Given a list of integers, use a vector and return the median (when sorted, the value in the middle
 * position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */
pub fn median_and_mode(numbers: &mut Vec<i32>) -> (f32, i32) {
    numbers.sort();
    let median = if numbers.len() % 2 == 0 {
        (numbers[numbers.len() / 2 - 1] + numbers[numbers.len() / 2]) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    let mut map = HashMap::new();

    for &mut number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mode = *map
        .iter()
        .max_by_key(|(_, &v)| v)
        .unwrap_or((&0, &0))
        .0;

    (median, mode)
}

/**
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
 * “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to
 * the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
pub fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first_char) if "aeiou".contains(first_char) => word.to_string() + "-hay",
        Some(first_char) => chars.collect::<String>() + "-" + &first_char.to_string() + "ay",
        None => String::new(),
    }
}

/**
 * Using a hash map and vectors, create a text interface to allow a user to add employee names to a
 * department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
 * the user retrieve a list of all people in a department or all people in the company by department,
 * sorted alphabetically.
 */
pub fn company_and_people() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        print!("Please enter a command:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["Add", name, "to", department] => {
                let department = department.to_string();
                let name = name.to_string();

                company.entry(department).or_insert(Vec::new()).push(name);
            }
            ["Retrieve", department] => {
                if let Some(names) = company.get_mut(*department) {
                    names.sort();
                    println!("{:?}", names)
                } else {
                    println!("Department not found");
                }
            }
            ["Retrieve"] => {
                let mut departments: Vec<_> = company.iter().collect();
                departments.sort_by(|a, b| a.0.cmp(b.0));
                for (department, names) in departments {
                    let mut names = names.clone();
                    names.sort();
                    println!("{}: {:?}", department, names);
                }
            }
            ["Exit"] => break,
            _ => println!("Invalid command!")
        }
    }
}
