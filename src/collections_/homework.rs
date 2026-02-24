use std::collections::HashMap;
use std::io::{self, Write};

pub fn stats(numbers: &mut [i32]) -> (f64, i32) {
    numbers.sort();
    let mid = numbers.len() / 2;
    let median = if numbers.len().is_multiple_of(2) {
        (numbers[mid - 1] + numbers[mid]) as f64 / 2.0
    } else {
        numbers[mid] as f64
    };

    let mut map: HashMap<i32, i32> = HashMap::new();
    for &num in numbers.iter() {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    let mode = map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Empty list");

    (median, mode)
}

pub fn pig_latin(text: &str) -> String {
    const VOVELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut result: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        let mut chars = word.chars();
        let first_char = match chars.next() {
            Some(c) => c,
            None => continue,
        };
        let transformed = if VOVELS.contains(&first_char.to_ascii_lowercase()) {
            format!("{word}-hay") // apple -> apple-hay
        } else {
            let rest: String = chars.collect();
            format!("{rest}-{first_char}ay") // first -> irst-fay
        };
        result.push(transformed);
    }

    result.join(" ")
}

pub fn company_app() {
    fn print_all(department: &str, names: &mut [String]) {
        names.sort();
        println!("Employees in {department}:");
        // iter to borrow the names instead of taking them
        for name in names.iter() {
            println!("- {}", name);
        }
    }

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Commands: ");
    println!("Add [name] to [department]");
    println!("List [department]");
    println!("All");
    println!("Quit");

    loop {
        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.starts_with("Add ") {
            let parts: Vec<&str> = input.trim()["Add".len()..].splitn(2, " to ").collect();

            match parts.as_slice() {
                [name, department] => {
                    let name = name.trim();
                    company
                        .entry(department.to_string())
                        .or_default()
                        .push(name.to_string());
                    println!("{name} added to the department {department}");
                }
                _ => println!("Invalid input"),
            }
        } else {
            let words: Vec<&str> = input.split_whitespace().collect();

            match words.as_slice() {
                ["List", department] => {
                    if let Some(names) = company.get(*department) {
                        print_all(department, &mut names.clone());
                    } else {
                        println!("Department not found");
                    }
                }
                ["All"] => {
                    let mut departments: Vec<_> = company.keys().collect();
                    departments.sort();
                    for department in departments {
                        let mut names = company.get(department).unwrap().clone();
                        print_all(department, &mut names);
                    }
                }
                ["Quit"] => break,
                _ => println!("Pattern not acceptable"),
            }
        }
    }
}
