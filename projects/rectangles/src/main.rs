#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width) 
    } 

    fn square(size: u32) -> Rectangle {
        Rectangle {
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

    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };

    let rect3 = Rectangle {
        width: 45, 
        height: 25,
    };

    let sq1 = Rectangle::square(28);
    let sq2 = Rectangle::square(35);

    println! ("rect1 可以装下 rect2 吗？{}", rect1.can_hold(&rect2));
    println! ("rect1 可以装下 rect3 吗？{}", rect1.can_hold(&rect3));
    println! ("rect1 可以装下 sq1 吗？{}", rect1.can_hold(&sq1));
    println! ("rect1 可以装下 sq2 吗？{}", rect1.can_hold(&sq2));
}
