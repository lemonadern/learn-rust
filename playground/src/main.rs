#![allow(unused)]

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let first = v.get(0);
    let sixth = v.get(5);

    fn log(e: Option<&i32>) {
        match e {
            None => println!("No Element."),
            Some(e) => println!("This is {:?}.", e),
        }
    }
    log(first);

    log(sixth);
}
