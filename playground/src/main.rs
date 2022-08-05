#![allow(unused)]

fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value{
        Some(3)=>println!("three"),
        _=>()
    }
    
    if let Some(3)= some_u8_value {
        println!("three");
    }

}
