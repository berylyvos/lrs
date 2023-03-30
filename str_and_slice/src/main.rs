fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hw = &s[..];
    let s = "数据库"; // 3 bytes per hanzi
    println!("{} | {} | {} | {}", hello, world, hw, &s[3..6])
}

fn string2str() {
    let s = String::from("hello,rust!");
    let s1 = &s;      // &String
    let s2 = &s[..];     // &str
    let s3 = s.as_str(); // &str
    say_hello(s1);
    say_hello(s2);
    say_hello(s3);
}

fn say_hello(s: &str) {
    println!("{}",s);
}

#[allow(dead_code)]
fn string_cannot_index() {
    let _s1 = String::from("hello");
    // the type `String` cannot be indexed by `{integer}`
    // the trait `Index<{integer}>` is not implemented for `String`
    // let h = _s1[0];
}

fn string_opt() {
    let mut s = String::from("Hello ");

    s.push_str("Jessica");
    s.push('!');
    say_hello(&s);
    s.insert(6, '@');
    say_hello(&s);

    let s1 = s.replace("Hello", "Bye");
    dbg!(s1);
    dbg!(s.pop());
    say_hello(&s);
    s.clear();
}

fn add() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("{}", result);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}

fn escape() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

fn main() {
    slice();
    string2str();
    string_opt();
    add();
    escape();
}