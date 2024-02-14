use std::{thread, time::Duration};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_closure = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

pub fn test_generate_workout() {
    generate_workout(10, 0);
    generate_workout(25, 3);
}

fn _capture_immut_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
}

fn _capture_mut_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn _move_ownership_to_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

static mut LIST: [Rectangle; 3] = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
];

pub fn sort_by_key_with_fnmut() {
    unsafe {   
        LIST.sort_by_key(|r| r.width);
        println!("{:#?}", LIST);
    }
}

fn _try_sort_by_key_with_fnonce() {
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    unsafe {
        LIST.sort_by_key(|r| {
            // sort_operations.push(value);
            r.width
        });
        println!("{:#?}", LIST);
    }
}

pub fn sort_by_key_with_fnmut_count() {
    let mut num_sort_operations = 0;
    unsafe {
        LIST.sort_by_key(|r| {
            num_sort_operations += 1; // mut ref
            r.height
        });
        println!("{:#?}, sorted in {num_sort_operations} operations", LIST);
    }
}