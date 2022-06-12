#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// this part implements methods for the Rectangle struct
impl Rectangle {
    // &self is short for "self: &Self", where Self is an alias for the struct type the impl belongs to
    fn area(&self) -> u32 {
        // self is the struct instance referenced by the method
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // methods without self are static methods
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

    let rect2 = Rectangle::square(25);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The rectangle has an area of {}", rect1.area());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}