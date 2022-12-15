#![allow(unused)]

use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(e) => {
                panic!("Tried to create file, but there was a problem: {:?}", e);
            }
        },
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err)
        }
    };
}

#[test]
fn it_works_everywhere() {
    assert_eq!(2 + 2, 4);
}

#[cfg(feature = "learning")]
#[test]
fn it_is_learning_test() {
    assert_eq!(2 + 2, 4);
}
