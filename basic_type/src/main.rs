fn integer_overflow() {
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
}

fn floating() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
}

fn floating_trap() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
}

fn range_emoji() {
    const EMO: char = '\u{1F605}';
    for i in 1..10 {
        print!("{}{} ", i, EMO)
    }
    println!("\nsize of {}: {} bytes", EMO, std::mem::size_of_val(&EMO));
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}

fn main() {
    integer_overflow();
    floating();
    floating_trap();
    range_emoji();
    assert_eq!(ret_unit_type(), ());
}
