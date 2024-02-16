use std::rc::Rc;
use crate::rc_t::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn test_refrence_counted_smart_pointer() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("a.strong_count: {}", Rc::strong_count(&a)); // 1
    
    let _b = Cons(3, Rc::clone(&a));
    println!("a.strong_count: {}", Rc::strong_count(&a)); // 2
    
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("a.strong_count: {}", Rc::strong_count(&a)); // 3
    }

    println!("a.strong_count: {}", Rc::strong_count(&a)); // 2
    println!("a.weak_count: {}", Rc::weak_count(&a)); // 0
}