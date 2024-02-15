#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn test_struct() {
    let p1 = Point{x: 3, y: 2.1};
    let p2 = Point{x: "Hello", y: '~'};
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);
}

enum _Option<T> {
    Some(T),
    None,    
}

enum _Result<T, E> {
    Ok(T),
    Err(E),
}