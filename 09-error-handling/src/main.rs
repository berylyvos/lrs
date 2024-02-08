use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>>{
    let _f = File::open("aaa.txt")?;
    Ok(())
}

fn _unrecoverable_error_with_panic() {
    let v = vec![0, 0, 0];
    v[42];
    // index out of bounds: the len is 3 but the index is 42
}

fn _recoverable_error_with_result() {
    let f = File::open("a.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error);
        }
    };
}

fn _handling_different_errors_with_match() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn _handling_different_errors_with_unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _expect() {
    let _f = File::open("a.txt").expect("cannot open file");
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}