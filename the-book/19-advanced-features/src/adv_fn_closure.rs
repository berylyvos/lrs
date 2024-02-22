// Function Pointers

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn test_func_ptr() {
    let ans = do_twice(add_one, 20);
    println!("The answer is: {}", ans);
}

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), 
// meaning you can always pass a function pointer as an argument for a function that expects a closure. 
// It’s best to write functions using a generic type and one of the closure traits so your functions can 
// accept either functions or closures.

fn _numbers_to_strings_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
}

fn _numbers_to_strings_fn_ptr() {
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
}

fn _initializer_function_as_fn_ptr() {
    enum Status {
        Value(u32),
        Stop,
    }

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Returning Closures

fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}