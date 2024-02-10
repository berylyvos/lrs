fn _preventing_dangling_references() {
    let r;
    {
        let _x = 5;
        // r = &_x; // borrowed value does not live long enough
    }
    let y = 2;
    r = &y;
    println!("{r}");
}

// the generic lifetime 'a will get the concrete lifetime that is equal to 
// the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn generic_lifetimes_in_func() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn _generic_lifetimes_in_func() {
    let s1 = String::from("long string is long");
    let result;
    {
        let _s2 = String::from("xyz");
        // result = longest(s1.as_str(), s2.as_str()); // `s2` does not live long enough
    }
    result = longest(&s1, "123");
    println!("The longest string is {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn lifetime_in_struct(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let ie = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", ie.part);
}


// lifetime elision
// *rule 1* : a function with 1 parameter gets 1 lifetime parameter
// *rule 2* : if there is exactly 1 input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// fn first_word<'a>(s: &'a str) -> &'a str
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        42
    }

    // There are two input lifetimes, so Rust applies the first lifetime elision rule 
    // and gives both &self and announcement their own lifetimes. Then, because one of
    // the parameters is &self, the return type gets the lifetime of &self, and all 
    // lifetimes have been accounted for.
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn _static_lifetime() {
    let _s: &'static str = "static string";
}