#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 13,
    };

    println!("{rect1:#?}");
    println!("area: {}", rect1.area());

    let rect2 = Rectangle::square(12);

    println!("{rect2:#?}");
    println!("area: {}", rect2.area());
}
