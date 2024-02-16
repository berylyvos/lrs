use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn treat_smart_pointers_like_regular_ref_with_deref() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, *y);
    assert_eq!(x, *(y.deref()));
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

pub fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    // &m &MyBox<String>
    // deref -> &String
    // deref -> &str
    hello(&m);
}

// Rust does deref coercion when it finds types and trait implementations in three cases:
//
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>