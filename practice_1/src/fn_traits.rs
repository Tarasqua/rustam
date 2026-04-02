fn main() {
    // let s = String::from("hell");
    // let f = move || s;

    // println!("{}", f());
    // println!("{}", f()); // ERROR: use of moved value

    // NOTE: instead
    let s = String::from("hello");
    let f = || s.clone(); // or &s

    println!("{}", f());
    println!("{}", f());

    // ===========================
    let mut s = String::from("hello");
    let f = move || s.push_str(" world!"); // or &s

    call_smth(f);
}

fn call_smth<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
