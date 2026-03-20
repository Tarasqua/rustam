// INFO:
// Different from references and smart pointers, raw pointers:
// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
// - Aren’t guaranteed to point to valid memory
// - Are allowed to be null
// - Don’t implement any automatic cleanup

use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;

fn raw_pointer_example() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // let address = 0x012345usize; // INFO: Creating a raw pointer to an arbitrary memory address
    // let r = address as *const i32; // WARNING: There might be data at that address or there might not, the compiler might optimize the code so that there is no memory access, or the program might terminate with a segmentation fault.

    unsafe {
        // INFO: we can’t dereference raw pointers and read the data being pointed to -> we must use unsafe blocks
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    };
}

fn danger() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

pub fn splitter() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // raw pointer with the type *mut i32

    assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..]) // INFO: borrow checker does not understand we're borrowing different parts of the slice, so we need unsafe to bypass it

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    // fn abs(input: i32) -> i32;
    safe fn abs(input: i32) -> i32;
}

pub fn call_abs() {
    // unsafe {
    //     let result = abs(-5);
    //     println!("abs(-5) is: {}", result);
    // }
    let result = abs(-5);
    println!("abs(-5) is: {}", result);
}

// Declare the external C function
unsafe extern "C" {
    fn system(command: *const c_char) -> i32;
}

pub fn ls() {
    let cmd = CString::new("ls").unwrap();
    unsafe {
        system(cmd.as_ptr());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    // C called Rust code here
    println!("Just called a Rust function from C!");
}

// INFO: A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Another difference is that static variables can be mutable
static HELLO_WORLD: &str = "Hello, World!";

pub fn hello_world() {
    println!("{HELLO_WORLD}");
}

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn count() {
    unsafe {
        // SAFETY: This is only called from a single thread in `count`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
