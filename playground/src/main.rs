use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T, parent: Weak<Node<T>>, children: Vec<Rc<Node<T>>>) -> Rc<Node<T>> {
        Rc::new(Node {
            value,
            parent: RefCell::new(parent),
            children: RefCell::new(children),
        })
    }

    fn set_parent(&self, parent: &Rc<Node<T>>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent)
    }

    fn set_child(&self, child: &Rc<Node<T>>) {
        self.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    let leaf = Node::new("xfy", Weak::new(), vec![]);

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Node::new("dfy", Weak::new(), vec![Rc::clone(&leaf)]);

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        leaf.set_parent(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
