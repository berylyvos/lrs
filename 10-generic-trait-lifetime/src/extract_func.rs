fn max(list: &[i32]) -> i32 {
    let mut max = &list[0];
    for x in list {
        if x > max {
            max = x;
        }
    }
    *max
}

pub fn test_max() {
    let v = vec![-1, 100, 0, 101, 1];
    let max = max(&v);
    println!("{max}");
}