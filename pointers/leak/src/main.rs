use std::cell::RefCell;
use std::rc::{Rc, Weak};

use leak::List::{Cons, Nil};
use leak::Node;

fn main() {
    let root = Node::new(1);

    let leaf_weak = Node::add_child(&root, 2); // leaf created inside the tree

    println!("Strong count root: {}", Rc::strong_count(&root)); // 1

    if let Some(leaf) = leaf_weak.upgrade() {
        // try to check if the child is still alive
        println!("Child is alive: {}", leaf.value);
    }

    // delete root -> children are dropped automatically
    drop(root);

    if leaf_weak.upgrade().is_none() {
        println!("Child is dropped along with root");
    }
}

fn cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail());

    // WARNING: this will create a cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // WARNING: Uncomment the next line to see that we have a cycle; it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None since leaf has no parent yet

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrade to weak reference
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Some(branch) since leaf now has a parent
}

fn another_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}", // 1 (itself) 0 (no parent)
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}", // 1 (itself) 1 (leaf)
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}", // 2 (itself + branch) 0 (no parent)
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None since leaf has no parent after branch goes out of scope
    println!(
        "leaf strong = {}, weak = {}", // 1 (itself), 0 (no parent)
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
