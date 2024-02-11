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
mod tests {
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
