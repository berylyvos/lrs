use std::{cell::RefCell, rc::Rc};
use crate::rc_refcell_t::List::{Cons, Null};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Null,
}

pub fn test_rc_refcell_list() {
    let v = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Null)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *v.borrow_mut() += 10;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}