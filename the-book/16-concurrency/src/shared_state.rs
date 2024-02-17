use std::{sync::{Mutex, Arc}, thread};

pub fn _test_mutex() {
    let m = Mutex::new(42);

    {
        let mut num = m.lock().unwrap();
        *num = 0;
    }

    println!("m = {:?}", m);
}

pub fn share_mutex_between_threads_using_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.lock().unwrap());
}