pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 3,
            height: 3,
        };

        let r2 = Rectangle {
            width: 1,
            height: 1,
        };

        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle {
            width: 3,
            height: 3,
        };

        let r2 = Rectangle {
            width: 1,
            height: 1,
        };

        assert!(!r2.can_hold(&r1));
    }

    // #[test]
    // #[ignore]
    // fn expensive_test() {
    //     // code that takes an hour to run
    // }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
