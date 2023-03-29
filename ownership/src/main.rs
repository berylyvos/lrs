fn ownership_move() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2

    println!("{}, world! {:?}", s2, s2.as_ptr());
}

fn copy() {
    let x: &str = "hello, rust";
    let y = x;
    println!("{:?}, {:?}", x.as_ptr(), y.as_ptr()); // same
}

fn clone() {
    let s1 = String::from("rusty");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s1 = {:?}, s2 = {:?}", s1.as_ptr(), s2.as_ptr());
}

fn copy_i32() {
    let x = 5;
    let px = &x as *const i32;
    let y = x;
    let py = &y as *const i32;

    println!("px = {:?}, py = {:?}", px, py);
}

fn unmutable_reference() {
    let s1 = String::from("hello");
    let s2 = &s1;
    let len = calculate_length(s2);

    println!("s1: {:?}, s2: {:?}", s1.as_ptr(), s2.as_ptr());
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    println!("s: {:?}", s.as_ptr());
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", shingo");
}

fn mutable_and_unmutable() {
   let mut s = String::from("SHINGO");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束

fn dangling_reference() {
    let _s = no_dangle();
}

/* 
fn dangle() -> &String {
    let s = String::from("hello");
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    ownership_move();
    copy();
    clone();
    copy_i32();
    unmutable_reference();
    mutable_reference();
    mutable_and_unmutable();
    dangling_reference();
}
