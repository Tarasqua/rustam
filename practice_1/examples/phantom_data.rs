#![allow(dead_code, unused)]

use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    let a = BytesRepr::<u32> {
        bytes: 42u32.to_be_bytes().to_vec(),
        p: PhantomData,
    };
}

struct BytesRepr<T> {
    // INFO: we need to use PhantomData to tell the compiler that T is used in this struct, while keeping the bytes field generic
    bytes: Vec<u8>,
    p: PhantomData<T>,
}

struct WithLifetime<'a> {
    // INFO: here we pass PhantomData to tell the compiler that the struct live as long as the lifetime 'a (the data)
    data: *const u8,
    p: PhantomData<&'a ()>,
}

struct NonSendSync {
    data: String,
    p: PhantomData<Rc<()>>, // INFO: PhantomData is used to tell the compiler that the struct is not Send or Sync, 'cause Rc<()>'s inner data is not Send or Sync
}
