#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return other.width < self.width && other.height < self.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(30);

    dbg!(&rect1);

    println!("The are of the rectangle is {} square pixels", rect1.area());
    println!("rect1 is {:#?}", &rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
