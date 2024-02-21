fn _deref_a_raw_pointer() {
    let mut num = 3;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 42;
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r: {}", *r);
    }
}

use std::slice;

// Creating a Safe Abstraction over Unsafe Code
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn test_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);

    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn test_call_external_code() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static HELLO: &str = "hello";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn test_static_var() {
    println!("{}", HELLO);
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}