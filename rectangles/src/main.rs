#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 25,
        height: 45,
    };
    
    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // note the borrow here to avoid taking ownership.
    );


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );

    let square = Rectangle::square(32);

    println!("{:?}", square)
}

// area as function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// area as method
impl Rectangle {
    // Self is an alias for the type of the impl block, Rectangle in this case. You can also just use the &self shorthand.
    fn area(self: &Self) -> u32 {
    // fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
