fn _test() {
    let some_option_value = Some(42);
    // let Some(x) = some_option_value; // pattern `None` not covered
    // note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    if let Some(x) = some_option_value {
        println!("{x}");
    }
}