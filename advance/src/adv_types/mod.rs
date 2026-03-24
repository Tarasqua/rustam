use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn aliases() {
    type Kilometers = i32;

    let x: i32 = 3;
    let y: Kilometers = 5;

    assert_eq!(x + y, 8);

    // ====================

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("Hi"))
    }
}

// NOTE: never (!) type is a special primitive type that represents the result of a computation that never complete: panic!(), continue, return, loop, std::process::exit()
// let x: u32 = match some_option {
//     Some(val) => val,
//     None => panic!("This will never return a u32, so it's fine!"),
// };
//
// ERROR: fn fatal_error(msg: &str) -> ! {
//     eprintln!("{}", msg);
//     std::process::exit(1);
// }
//
// NOTE: Result<T, !> to indicate that an operation cannot fail

// INFO: the function bar returns never (! - never type)
fn bar() -> ! {
    panic!();
}

fn forever() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

// INFO: DST (Dynamically Sized Types)
// NOTE: The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
