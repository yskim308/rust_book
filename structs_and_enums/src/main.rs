struct Rectangle {
    width: u32,
    height: u32,
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
    let rectangle1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rectangle2 = Rectangle {
        width: 25,
        height: 10,
    };

    println!("rectangle 1 area: {}", rectangle1.area());
    println!("rectangle 2 area : {}", rectangle2.area());

    println!("1 can fit 2? {}", rectangle1.can_hold(&rectangle2));
}
