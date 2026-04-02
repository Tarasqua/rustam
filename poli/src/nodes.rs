use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // INFO:
    // Parent -> Child: Strong reference (Rc). The parent "owns" the children.
    // Child -> Parent: Weak reference (Weak). The child "observes" the parent.
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: Default::default(),
            children: Default::default(),
        })
    }

    fn set_parent(&self, parent: Rc<Node>) {
        let weak_parent = Rc::downgrade(&parent); // INFO: downgrade to Weak to avoid circular references
        *self.parent.borrow_mut() = Some(weak_parent);
    }

    fn add_child(self: &Rc<Self>, child: Rc<Node>) {
        child.set_parent(self.clone());
        self.children.borrow_mut().push(child);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!(
            "Dropping node with value {} and {} children",
            self.value,
            self.children.borrow().len()
        );
    }
}

pub fn nodes_() {
    let tree = Node::new(1);
    tree.add_child(Node::new(3));
    tree.add_child(Node::new(5));
    println!("Finishing program now");
}
