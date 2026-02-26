use std::fmt::Display;

use super::common::largest;

// fn longest(x: &str, y: &str) -> &str { // won't work since the function signature doesn't specify the lifetime of the references (Lifetime Elision Rules not applied)
//     if x.len() > y.len() { x } else { y }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // INFO: the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y
    if x.len() > y.len() { x } else { y }
}

// fn one_more_longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // INFO: fail to compile because the return value lifetime is not related to the lifetime of the parameters at all
// }

fn lifetimes() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x; // r is a reference to x, but x goes out of scope at the end of this block
    // }

    // println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let string_1 = String::from("long string is long");

    {
        let string_2 = String::from("xyz");
        let longest_result = longest(string1.as_str(), string_2.as_str());
        println!("The longest string is {longest_result}");
    }

    // string_2 and longest_result go out of scope here, so we can't use them anymore
    println!("{string_1}");

    // let string_3 = String::from("long string is long");
    // let result;
    // {
    //     let string_4 = String::from("xyz");
    //     result = longest(string_3.as_str(), string_4.as_str()); // INFO: string_4 goes out of scope at the end of this block, so result will have a reference to a string that is no longer valid
    // }
    // println!("The longest string is {result}");
}

// INFO: This one works because of three rules of lifetimes (Lifetime Elision Rules):
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.
//
// Rule 1: fn first_word<'a>(s: &'a str) -> &str { ... }
// Rule 2: fn first_word<'a>(s: &'a str) -> &'a str { ... }
// Rule 3 isn't used here
//
// So that all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first elision rule is applied
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // third lifetime elision rule applies and the lifetime of self is assigned to the return value
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn expert() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn make_announcement() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {result}");
}
