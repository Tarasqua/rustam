use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int32(i32),
    Float64(f64),
    Text(String),
}

pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(1);
    v.push(1);
    // v.push('a');

    let third: &i32 = &v[2];
    dbg!(third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third value ir {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // adding a reference lead to a borrow error
    v[0] = 10; // and this line is invalid
    v.push(1); // and this one too
    println!("The first element is: {first}");

    let mut v = vec![100, 52, 23];
    for i in &mut v {
        *i += 50; // dereference
    }
    dbg!(v);

    let mut row = vec![
        SpreadsheetCell::Int32(32),
        SpreadsheetCell::Float64(32.),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Text("Hello".to_string()),
    ];
    let last = row.last().unwrap(); // without deleting the last element
    dbg!(last);
    let last = row.pop(); // removes and gives the Some statement
    dbg!(last);
}

pub fn strings() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {s2}"); // s2 is bar
    let s3 = s2.to_owned() + "hello";

    let mut s = String::from("lo");
    s.push('l');

    let input = "10 20 30";
    let total: i32 = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum(); // Result: 60

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + operator works as fn add(self, s: &str) -> String {

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3; // not able to use s1 anymore
    let s = format!("{s1}-{s2}-{s3}");

    let hello = "Здравствуйте";
    let third = hello.chars().nth(2);
    dbg!(third); // Some('р')
    let third = hello.as_bytes()[2]; // u8
    let s = &hello[0..4];
    // "Зд" since slice returns first 4 bytes and each unicode scalar value is 2 bytes
    println!("{s}");
    // let panic_slice = &hello[0..1]; // panic: byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}"); // 208 151 208 180
    }

    let original_string = "Hello, world!".to_string();
    match original_string.contains("hello") {
        c => {
            original_string.replace("Hello", "Bye");
        }
        _ => {
            original_string.replace("world", "fellas");
        }
    }
    if let c = original_string.contains("hello") {
        original_string.replace("Hello", "Bye");
    }
}

pub fn hashes() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}"); //Yellow: 50\nBlue: 10
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    println!("{field_name}"); // name and value are moved to map so that we can't use them anymore
    // println!("{field_value}"); // value is moved to map so that we can't use them anymore
    let v: u32 = 5;
    let mut new_map = HashMap::new();
    new_map.insert(String::from("value"), v);
    println!("{v}"); // still valid since v is copied into new_map

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Blue is now 25 since 10 was overwritten

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // Yellow -> 50 is inserted
    scores.entry(String::from("Blue")).or_insert(50); // Blue -> 10 since entry condition is not met
    println!("{scores:?}"); // {"Yellow": 50, "Blue": 10}

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference (&mut V)
        // to the value for the specified key
        *count += 1;
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }
    println!("{map:?}");
}
