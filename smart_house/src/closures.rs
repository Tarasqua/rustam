#![allow(unused_variables)]

#[derive(Debug)]
pub struct S;

pub fn closures() {
    let mut q = S;
    let r = S;
    let x = move |x: S| drop(x); // INFO: FnOnce
    let y = |x: S| q = x; // INFO: FnMut + FnOnce
    let z = |x: S| println!("{x:?}");

    accepts_fn_once(x);
    accepts_fn_mut(y);
    accepts_fn(z);
}

fn accepts_fn_once<F: FnOnce(S)>(f: F) {
    f(S);
}

fn accepts_fn_mut<F: FnMut(S)>(mut f: F) {
    f(S);
}

fn accepts_fn<F: Fn(S)>(f: F) {
    f(S);
}
