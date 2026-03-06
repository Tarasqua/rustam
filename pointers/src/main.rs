#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;
use pointers::ListRc::{Cons as RcCons, Nil as RcNil};
use pointers::ListRcRefCell::{Cons as RcRefCellCons, Nil as RcRefCellNil};
use std::cell::RefCell;
use std::{mem::size_of_val, rc::Rc};

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);
    println!("size of list: {}", size_of_val(&list));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("size of list: {}", size_of_val(&list));

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created"); // "Dropping ..." after main exits in reverse order
    // d.drop(); // WARNING: not allowed since drop is called automatically when the value goes out of scope -> lead to double free
    drop(c); // OK: drop is called manually, not automatically

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2

    let value = Rc::new(RefCell::new(5)); // INFO: Rc<RefCell<i32>> allows shared ownership of a mutable value
    let a = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::new(RcRefCellNil)));
    let b = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::clone(&a)));
    let c = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::clone(&a)));

    *value.borrow_mut() += 10; // INFO: dereference operator (*) to access the inner RefCell, then call borrow_mut() to get a mutable reference to the value inside

    println!("a after = {a:?}"); // a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {b:?}"); // b after = Cons(RefCell { value: 15 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {c:?}"); // c after = Cons(RefCell { value: 15 }, Cons(RefCell { value: 15 }, Nil))
}
