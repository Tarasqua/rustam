use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct MyStruct<'a> {
    a: i32,
    b: &'a str,
}

fn main() {
    let s = MyStruct { a: 42, b: "hello" };
    let str = serde_json::to_string(&s).unwrap();
    println!("{}", str);
    assert_eq!(s, serde_json::de::from_str(&str).unwrap());
}
