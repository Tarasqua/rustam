mod rectangle;
use crate::rectangle::run;
mod enuming;
use crate::enuming::enuming;

use rustam::collections_;
use rustam::garden;
use rustam::library;
use rustam::utils;

use std::{cmp::Ordering, collections::HashMap, fmt, io};

use rand::Rng;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Args {
    a: String,
    b: String,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn fun(Args { a, b }: Args) {
    println!("a: {}, b: {}", a, b);
}

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

fn main() {
    rustam::greet();
    collections_::common::vectors();
    collections_::common::strings();
    collections_::common::hashes();

    rustam::tsc_utils_test();
    utils::make_log("main");

    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("k", "v");
    map.insert("another_key", "val");

    let plant = garden::vegetables::Asparagus {};
    let fruit = garden::fruits::Orange {};
    dbg!(plant);
    dbg!(fruit);

    run();
    enuming();
    let s = String::from("hello world bitches");
    let first_ = first_word(&s);

    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);

    let mut user = User {
        active: true,
        username: String::from("test"),
        email: String::from("test@example.com"),
        sign_in_count: 1,
    };
    user.email = String::from("another@example.com");

    let mut user1: User = build_user(String::from("value"), String::from("Username"));
    let user2: User = User {
        email: String::from("another"),
        ..user1
    };

    user1.username = String::from("value");

    println!("{}, {}", user1.username, user2.username);

    fun(Args {
        b: "b".into(),
        a: String::from("a"),
    });

    let black: Color = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;

    let subject = AlwaysEqual;
    // first_.clear();
    // reference();
    // let s1 = String::from("t");
    // let mut s2 = s1;
    // s2.push_str("string");
    // loop_();
    // another(23, '!');
    // // let tup: (i32, f64, bool, String) = (1, 2.9, true, "Hello".to_string());
    // // let tuple = (1, 2.5, false);
    // // let (x, y, z) = tuple;

    // // let arr: [i32; 3] = [1, 2, 3];
    // // let arr: [u8; 5] = [1; 5];
    // // let first = arr[0];

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     println!("Please input your guess.");
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse::<u32>() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}

fn build_user(email: String, username: String) -> User {
    let sign_in_count: u64 = 1;
    User {
        active: true,
        username,
        email,
        sign_in_count,
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn another(x: u8, u: char) {
    let a = sum_(x);
    let y = {
        let z = x + u as u8;
        z
    };
    println!("Another {} {}", y, a)
}

fn sum_(x: u8) -> u8 {
    let condition = x.is_multiple_of(2);
    // let condition = x % 2 == 0;
    let number = if condition { 1 } else { 2 };
    println!("Number is {}", number);

    if x > 5 {
        println!("x is greater than 5");
        x + 1
    } else if x == 5 {
        x + 2
    } else {
        x + 3
    }
    // 5 + x
}

fn loop_() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let a = &[1; 5];
    for element in a {
        println!("Element is {}", element)
    }

    for number in (0..4).rev() {
        println!("Number is {}", number)
    }
    // let mut counter: u32 = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("Result is {}", result);
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iter(n: u32) -> u32 {
    let mut s = String::from("test");
    takes_ownership(s);
    // s.push_str("test");

    if n <= 1 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let next = a + b;
            a = b;
            b = next;
        }
        b
    }
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn gives_ownership() -> String {
    String::from("test")
}

fn reference() {
    let s = String::from("est");
    calculate_length(&s);

    let mut s_ = String::from("test");
    calculate_length_(&mut s_);

    let r1 = &mut s_;
    let r2 = &mut s_;
}

fn calculate_length(s: &str) -> usize {
    // s.push_str("test");
    s.len()
}

fn calculate_length_(s: &mut String) -> usize {
    s.push_str("string");
    s.len()
}

fn multiple_references() {
    let mut s = String::from("test");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
