#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let w = 100;
    let h = 50;
    let r = Rectangle {
        width: w,
        height: h,
    };
    println!("{}", area(&r));
    println!("{:#?}", r);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
