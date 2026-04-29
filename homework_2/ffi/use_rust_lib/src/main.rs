// check build.rs
unsafe extern "C" {
    fn add_numbers_c(left: u64, right: u64) -> u64;
}

fn main() {
    let a = 2;
    let b = 3;
    // let d = synt_ex_lib::add_numbers_c(a, b);
    // println!("{a} + {b} = {d}");
    let c = unsafe { add_numbers_c(a, b) }; // using as static lib
    println!("{a} + {b} = {c}");
}
