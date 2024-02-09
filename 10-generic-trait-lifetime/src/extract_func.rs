fn max<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut max = &list[0];

    for x in list {
        if x > max {
            max = x;
        }
    }

    max
}

pub fn test_max() {
    let v = vec![-1, 100, 0, 101, 1];
    let max_num = max(&v);
    println!("{max_num}");

    let v = vec![String::from("hello"), String::from("world")];
    let max_str = max(&v);
    println!("{max_str}");
}