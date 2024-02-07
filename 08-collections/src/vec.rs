pub fn new_vec() {
    let empty_vec: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:#?}", empty_vec);
    println!("{:#?}", v);
}

pub fn update_vec() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}

pub fn read_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The 3rd element is {}", third);

    match v.get(100) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no 3rd element!")
    }
}

pub fn iterate_vec() {
    let mut v = vec![0, -2147483648, 42];
    for e in &mut v {
        *e += 1;
        println!("{}", e);
    }
}

pub fn enum_vec() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row);
}