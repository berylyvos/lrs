struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: false,
    }
}

fn main() {
    test_structs();
    test_rectangle();
}

fn test_structs() {
    let mut user1 = User {
        email: String::from("shingo@gnix.com"),
        username: String::from("Shingo"),
        sign_in_count: 223,
        active: false,
    };

    user1.active = true;
    user1.sign_in_count += 1;

    let user2 = build_user(user1.email, user1.username);
    user1.email = String::from("new@xxx.com");
    println!("{}, {}", user1.email, user2.username);

    let user3 = User {
        email: String::from("user3@xxx.com"),
        ..user2
    };
    println!("{}", user3.username);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}

fn test_rectangle() {
    let rect1 = Rectangle{
        width: 22,
        length: 33,
    };
    let rect2 = Rectangle{
        width: 33,
        length: 44,
    };
    let rect3 = Rectangle{
        width: 44,
        length: 33,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:#?}", rect1);

    println!("{}", rect2.can_hold(&rect1));
    println!("{}", rect3.can_hold(&rect1));

    let square = Rectangle::square(0x7F);
    println!("{:#?}", square);
}