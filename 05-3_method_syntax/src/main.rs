#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Multiple `impl` blocks per struct are allowed
impl Rectangle {
    // We can choose whether we borrow or own self, mutable or not
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can have the same name as fields
    fn width(&self) -> bool {
        // Pretty bad design =(
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // To be used like: Rectangle::square(32);
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    if rect1.width() {
        println!("The rectangle has a nonzero width, it is: {}", rect1.width);
    }
    println!("The are of rect1 is {} square pixels", rect1.area());

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect3 = Rectangle::square(100);

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3?: {}", rect2.can_hold(&rect3));

    let square1 = Rectangle::square(24);
    println!("Can rect1 hold square?: {}", rect1.can_hold(&square1));
}
