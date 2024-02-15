pub fn new_string() {
    let s1 = "literal string".to_string();
    let s2 = String::from("This is what it is");
    println!("{}", s1);
    println!("{}", s2);
}

pub fn string_from_utf8() {
    let  hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    println!("{}", hello); // السلام عليكم
}

pub fn update_string() {
    let mut s = String::from("foo");
    let t = String::from("bar");
    s.push_str(&t);
    s.push('~');
    println!("{s}, {t}");
}

pub fn concat_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s2);
    println!("{}", s3);
}

pub fn concat_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

pub fn no_indexing() {
    let hello = "Здравствуйте";
    // let answer = &hello[0];
    println!("{}", hello.len());
    // [208, 151, 208, 180, 209, 128, 208, 176, 208, 178, 209,
    // 129, 209, 130, 208, 178, 209, 131, 208, 185, 209, 130, 208, 181]

    // З
    // д
    for c in "Зд".chars() {
        println!("{c}");
    }
}

pub fn slicing_string() {
    let hello = "Здравствуйте";
    let s = &hello[0..8];
    println!("{s}"); // Здра
}