#![allow(unused)]

struct User {
    username: String,
    email: String,
    age: u16,
    active: bool,
    sign_in_count: u64
}

fn main(){
    let user1 = User{
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        age: 20,
        active:true,
        sign_in_count:3
    };

    let user2 = User{
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("{}", user1.username)
}