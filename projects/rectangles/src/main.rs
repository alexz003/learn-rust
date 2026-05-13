#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    dbg!(&rect1);

    println!("The area of the rectangle {rect1:#?} is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 29,
        height: 49
    };

    match rect1.can_hold(&rect2) {
        true => {
            println!("rect1 can hold rect2");
        },
        false => {
            println!("rect1 can not hold rect2");
        }
    }
}
