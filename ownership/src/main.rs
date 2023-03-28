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

fn main() {
    ownership_move();
    copy();
    clone();
    copy_i32();
}
