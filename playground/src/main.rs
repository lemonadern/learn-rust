#![allow(unused)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
#[derive(Debug)]
enum State{
    Alabama,
    California
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}",state);
            25
        },
    }
}

fn main() {
    let c = Coin::Quarter(State::California);
    let value = value_in_cents(c);
    println!("{}", value);
}
