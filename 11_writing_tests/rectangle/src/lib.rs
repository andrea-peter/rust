#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        if other.width > 100 || other.height > 100 {
            panic!("Other rectangle too big!");
        }
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Other rectangle too big!")]
    fn panic_if_other_rect_too_big() {
        let rect = Rectangle {
            width: 8,
            height: 7,
        };
        let big_rect = Rectangle {
            width: 200,
            height: 7,
        };
        rect.can_hold(&big_rect);
    }
    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
