use std::collections::HashMap;

pub fn new_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:#?}", scores);
}

pub fn new_hashmap_from_vec() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![30, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("{:#?}", scores);
}

pub fn ownerships() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Black");

    let mut map = HashMap::new();
    map.insert(field_name, &field_value);
    // println!("{}", field_name); // field_name moved
    println!("{}", field_value);
}

pub fn accessing() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}

pub fn inserting() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

pub fn update_on_old_val() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}