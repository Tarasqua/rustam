use std::{cell::RefCell, ops::Deref, rc::Rc};

#[allow(dead_code)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // INFO: &self in input -> pointer to the MyBox<T> instance (where T is stored)
    // &self.0 in output -> pointer to the first element of the tuple (the value of type T)
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ListRcRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    Nil,
}

use ListRc::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dereference() {
        let x = 5;
        let y = &x;
        let z = Box::new(x);
        let m = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *z);
        assert_eq!(5, *m); // the same as *(m.deref())
    }

    #[test]
    fn test_hello() {
        let name = MyBox::new(String::from("Alice"));
        // INFO: deref coercion
        hello(&name); // &name coerces to &MyBox<String> which dereferences to &String and then dereferences to &str
        // INFO: if no deref coercion, we would need to write
        hello(&(*name)[..]);
        // WARNING: Immutable references will never coerce to mutable references

        assert_eq!(*name, "Alice");
    }

    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        // INFO: Rc::clone does NOT deep copy the data, it just increments the reference count
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }
}
