use libloading::{Library, Symbol};

type AddNumbersFn = unsafe extern "C" fn(left: u64, right: u64) -> u64;

fn main() {
    unsafe {
        let lib = Library::new("./target/debug/libsynt_ex_lib.so").unwrap();
        let add_numbers_fn: Symbol<'_, AddNumbersFn> = lib.get(b"add_numbers_c").unwrap();
        let a = 2;
        let b = 3;
        let c = add_numbers_fn(a, b);
        println!("{a} + {b} = {c}");
    }
}
