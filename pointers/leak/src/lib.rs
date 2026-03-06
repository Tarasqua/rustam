use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Node {
    // INFO: parent holds a strong reference to the child, while children hold weak references to the parent; so child should not own the parent, and if a child is dropped, the parent should not be affected; instead, if the parent is dropped, all children should be dropped as well
    // WARNING: making a parent with Rc<Node> will cause a memory leak via reference cycles
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(parent_rc: &Rc<Node>, value: i32) -> Weak<Node> {
        let new_node = Rc::new(Node {
            value,
            parent: RefCell::new(Rc::downgrade(parent_rc)),
            children: RefCell::new(vec![]),
        });
        parent_rc.children.borrow_mut().push(Rc::clone(&new_node)); // parent owns the child via Rc
        Rc::downgrade(&new_node) // return a weak reference to the child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
