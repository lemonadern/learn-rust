#[derive(Debug)]

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
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r = Rectangle {
        width: 100,
        height: 50,
    };
    let r2 = Rectangle {
        width: 90,
        height: 30,
    };

    let r3 = Rectangle {
        width: 100,
        height: 30,
    };

    println!("{}", r.area());

    println!("r is {:#?}", r);
    println!("r2 is {:#?}", r2);
    println!("r3 is {:#?}", r3);

    println!("`r` can hold `r2` ? {}", r.can_hold(&r2));
    println!("`r` can hold `r3` ? {}", r.can_hold(&r3));

    println!("{:?}", Rectangle::square(10));
}
