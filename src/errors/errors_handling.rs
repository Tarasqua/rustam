use std::{
    fs::{File, read_to_string},
    io::{Error, ErrorKind, Read},
    net::IpAddr,
};

pub fn erroring() {
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Panic creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    // the same with closures
    let username_file = File::open("username.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("username.txt").unwrap_or_else(|error| {
                panic!("Panic creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // let another_file = File::open("another.txt").unwrap(); // file on Ok and panic on Err
    // let another_file = // expect allows to provide a custom error message while working as unwrap
    //     File::open("another.txt").expect("another.txt should be included in this project");

    let username = match read_username_from_file("test.txt") {
        Ok(name) => name,
        Err(_) => "default".to_string(),
    };
    println!("Username: {username}");

    if let Some(c) = last_char_of_first_line("the last char\nof the first line is") {
        println!("last is {c}")
    } else {
        println!("No last char found")
    };

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid"); // should never fail
}

pub fn closures() {
    // anonymous functions that can capture variables from their surrounding scope
    // A simple closure with one parameter
    let add_one = |x| x + 1;
    println!("Result: {}", add_one(5)); // Prints "Result: 6"

    // A closure with no parameters and a block body
    let greeting = || {
        println!("Hello, world!");
    };
    greeting();
}

fn read_username_from_file(filename: &str) -> Result<String, Error> {
    // let username_file_result = File::open(filename);
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut username_file = File::open(filename)?; // the same as above

    let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // };
    username_file.read_to_string(&mut username)?;

    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?; // or even simpler in 3 strings

    Ok(username)
}

fn read_username_from_file_fs(filename: &str) -> Result<String, Error> {
    read_to_string(filename) // using fs::read_to_string
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

enum OurError {
    Io(std::io::Error),
    Format(String),
}

impl From<std::io::Error> for OurError {
    fn from(err: std::io::Error) -> Self {
        OurError::Io(err)
    }
}

fn read_username_from_file_custom(filename: &str) -> Result<String, OurError> {
    // The ? here sees an io::Error, looks at the return type (OurError),
    // finds your 'From' implementation, and calls it automatically.
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn find_user_id() -> Result<i32, String> {
    let list = vec![1, 2, 3];

    // .ok_or() turns Option<i32> into Result<i32, String>
    // If it's None, it returns the error string provided.
    let id = list.get(10).ok_or("User ID not found in list")?;
    Ok(*id)
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // We can use '?' on an I/O error
//     let content = fs::read_to_string("config.txt")?;

//     // AND we can use '?' on a Parse error in the same function!
//     let number: i32 = content.trim().parse()?;

//     println!("Success: {}", number);

//     Ok(()) // Must return this at the end
// }
