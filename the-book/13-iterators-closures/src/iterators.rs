 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn iter_next() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iter_sum() {
        let v1 = vec![1, 2, 3];
        let mut iter = v1.iter();
        iter.next();
        let totoal: i32 = iter.sum();
        assert_eq!(totoal, 5);
    }

    #[test]
    fn iter_map() {
        let v1 = vec![-1, 0, 1];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![0, 1, 2]);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = _shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );

    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::_new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn other_iterator_trait_methods() {
        let sum: u32 = Counter::_new()
            .zip(Counter::_new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        // 1  2  3   4  5
        // 2  3  4   5
        // 2  6  12  20
        //    6  12
        assert_eq!(18, sum);
    }
 }

 #[derive(PartialEq, Debug)]
 struct Shoe {
    size: u32,
    style: String,
 }

 fn _shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
 }

 struct Counter {
    count: u32
 }

 impl Counter {
     fn _new() -> Counter {
        Counter { count: 0 }
     }
 }

 impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
 }