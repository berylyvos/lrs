use std::fmt;

type _Thunk = Box<dyn Fn() + Send + 'static>;

fn _take_long_type(_f: _Thunk) {

}

fn _returns_long_type() -> _Thunk {
    Box::new(|| println!("hi"))
}

type _Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> _Result<usize>;
    fn flush(&mut self) -> _Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> _Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> _Result<()>;
}

// The Never Type that Never Returns

fn _foo() -> ! {
    loop {

    }
}

fn _bar() -> ! {
    let guess = "";
    loop {
        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

// Dynamically Sized Types

fn _str_is_dynamically_sized_types() {
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

// Sized Trait

fn _generic<T>(_t: T) {}

fn __generic<T: Sized>(_t: T) {}

fn ___generic<T: ?Sized>(_t: &T) {}