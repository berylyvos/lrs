fn mutable_var() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn unused_var() {
    let _x = 2;
    let _y = 3;
    // let y = 3; 
    // ^ help: if this is intentional, prefix it with an underscore: `_y`
}

// 变量解构
fn deconstruct() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    
    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e: i32
}

// 解构式赋值
fn deconstructive_assignment() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// 变量遮蔽
fn shadowing() {
    // 字符串类型
    let spaces = "   ";
    // let mut spaces = "   ";
    // spaces = spaces.len(); // mismatched types expected `&str`, found `usize`
    
    // usize数值类型
    let spaces = spaces.len();
    println!("spaces: {}", spaces)
}

fn main() {
    unused_var();
    mutable_var();
    deconstruct();
    deconstructive_assignment();
    shadowing();
}
