use std::{rc::Rc, cell::RefCell};
use crate::cycle_ref::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn test_cycle_reference() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a->next = {:?}", a.next());

    // b -> a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b->next = {:?}", b.next());

    // b <-> a
    if let Some(link) = a.next() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a.next = {}", Rc::strong_count(&b));
    println!("a rc count after changing a.next = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a->next = {:?}", a.next());
}

