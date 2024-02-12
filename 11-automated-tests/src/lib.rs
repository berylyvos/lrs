#[derive(Debug)]
struct _Rectangle {
    _width: u32,
    _height: u32,
}

impl _Rectangle {
    fn _can_hold(&self, other: &Self) -> bool {
        self._width > other._width && self._height > other._height
    }
}

#[cfg(test)]
mod test_rectangle {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = _Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = _Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = _Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = _Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger));
    }
}

pub struct Guess {
    _value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1, got {}", value)
        }
        else if value > 100 {
            panic!("Guess value must be <= 100, got {}", value)
        }
        Guess { _value: value }
    }
}

#[cfg(test)]
mod test_guess {
    use crate::Guess;

    #[test]
    #[should_panic(expected = "Guess value must be <= 100")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "Guess value must be >= 1")]
    fn less_than_1() {
        Guess::new(0);
    }
}

pub fn add_two(a: i32) -> i32 {
    _internal_adder(a, 2)
}

fn _internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test_result {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if _internal_adder(1, 2) == 3 {
            Ok(())
        } else {
            Err(String::from("1 + 2 != 3"))
        }
    }
}