fn main() {
    string_type();
    do_move();
    do_clone();
    ownership_and_func();
    return_value_and_scope();
    borrowing();
    borrow_mutable();
    one_mutable_ref();
    find_first_word();
}

fn find_first_word() {
    let s = String::from("Rust You Up");
    let first = first_word(&s);
    let s_literal = "yojoU";
    println!("{first}, {}", first_word(s_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}


fn one_mutable_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}


fn borrow_mutable() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn return_value_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                                // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("{s1}, {s3}")
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn ownership_and_func() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);      // s's value moves into the function...
                                         // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);     // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Types that implement Copy trait:
//
// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

fn do_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}");
}

fn do_move() {
    let s1 = String::from("hello");
    let s2 = s1; // move

    // println!("{s1}"); // s1 is invalid
    println!("{s2}");
}

fn string_type() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`
}
