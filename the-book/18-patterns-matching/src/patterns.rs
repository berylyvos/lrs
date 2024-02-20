// Mixing if let, else if, else if let, and else
pub fn if_let_else_if_let_else() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

pub fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn _destructure_a_tuple() {
    let (_a, _b, _c, _d) = ((), 0, "a", Some(1));
}

fn print_coordinates((x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}

pub fn func_param_destructure_a_tuple() {
    let point = (10, 2);
    print_coordinates(&point);
}