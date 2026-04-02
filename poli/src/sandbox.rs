use std::{collections::HashMap, ops::Deref};

struct Inventory {
    items: Vec<String>,
}

impl Deref for Inventory {
    type Target = [String]; // INFO: slice is the most "primitive" version of a contiguous sequence

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

pub fn sandbox() {
    let mut map = HashMap::new();
    let r1: &i32 = map.entry(5).or_insert(10);
    let r2: &i32 = map.entry(10).or_insert(20);
    dbg!(map);

    let a: Vec<i32> = Vec::new();

    let boxed_string = Box::new(String::new());
    assert_eq!(boxed_string.len(), 0); // len is possible because String implements the Deref trait

    let string = "Let's count characters in this string";
    let mut map = HashMap::new();
    for ch in string.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    }

    let iter_map = map.iter().collect::<Vec<_>>();

    let inventory = Inventory {
        items: vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ],
    };
    // Because of Deref, we can call .iter() directly on inventory
    // even though .iter() is a method of [String], not Inventory.
    for item in inventory.iter() {
        println!("You have a: {}", item);
    }

    // You can also use other slice methods
    println!("Total items: {}", inventory.len());
    println!("First item: {:?}", inventory.first());
}
