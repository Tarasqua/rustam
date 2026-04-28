use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use utils::profiling_::{product, product_long};

// NOTE: cargo flamegraph --freq 9999 -- examples/profiling_.rs
fn main() {
    let (n1, n2) = read_params();
    dbg!(n1, n2);

    let a = product(n1);
    let b = product_long(n2);

    println!("{a}, {b}");
}

fn read_params() -> (u64, u64) {
    let file = BufReader::new(File::open("target/params.txt").unwrap());
    let mut lines = file.lines();
    let n1 = dbg!(lines.next().unwrap().unwrap().trim())
        .replace("_", "")
        .parse()
        .unwrap();
    let n2 = lines
        .next()
        .unwrap()
        .unwrap()
        .replace("_", "")
        .trim()
        .parse()
        .unwrap();
    (n1, n2)
}
