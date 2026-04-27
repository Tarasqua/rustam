use std::{thread, time::Duration};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// NOTE: use rayon::prelude::* only for parallel iterators
// rayon::join(...)	   No	  Free module function.
// rayon::scope(...)   No	  Free module function.
// .par_iter()	       Yes	  Method from trait IntoParallelIterator.
// .par_map(...)	   Yes	  Method from trait ParallelIterator.
// .par_sort()	       Yes	  Method from trait ParallelSlice.

fn main() {
    let start = std::time::Instant::now();
    let a: u32 = 42;
    let (iresult, fresult) = rayon::join(|| inc_int(&a), || inc_float(&(a as f32)));
    println!(
        "int: {}, float: {} (elapsed time: {:?})",
        iresult,
        fresult,
        start.elapsed()
    ); // ~2s

    const NUM_OPERATIONS: u32 = 10;
    let one_sec = Duration::from_secs(1);
    let start = std::time::Instant::now();
    rayon::scope(|scope| {
        for _ in 0..NUM_OPERATIONS {
            scope.spawn(|_| thread::sleep(one_sec));
        }
    });
    println!(
        "{} 1-sec operations elapsed in {:?}",
        NUM_OPERATIONS,
        start.elapsed()
    ); // ~1s

    let start = std::time::Instant::now();
    let sum: i32 = vec![1; 100]
        .par_iter() // parallel iterator
        .inspect(|_| thread::sleep(one_sec)) // run a closure on each element
        .sum(); // sum the elements
    println!("Sum: {} (elapsed time: {:?})", sum, start.elapsed()); // ~4s
}

fn inc_int(a: &u32) -> u32 {
    thread::sleep(Duration::from_secs(1));
    println!("Int increased");
    a + 1
}

fn inc_float(a: &f32) -> f32 {
    thread::sleep(Duration::from_secs(2));
    println!("Float increased");
    a + 0.5
}
