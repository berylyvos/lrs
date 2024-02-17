use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    _val: i32,
    _parent: RefCell<Weak<Node>>,
    _children: RefCell<Vec<Rc<Node>>>,
}

pub fn test_weak_reference() {
    let leaf = Rc::new(Node {
        _val: 3,
        _parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            _val: 5,
            _parent: RefCell::new(Weak::new()),
            _children: RefCell::new(vec![Rc::clone(&leaf)]), // leaf strong_ref +1
        });

        *leaf._parent.borrow_mut() = Rc::downgrade(&branch); // branch weak_ref +1
        
        println!("branch's children = {:#?}", branch._children.borrow());

        println!("branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf));
    }

    println!("leaf's parent = {:#?}", leaf._parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf));
}