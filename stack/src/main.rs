use std::ops::{Deref, DerefMut};

#[derive(Debug)]

struct Stack<T>(Vec<T>);
#[derive(Debug)]
struct Guard<'a, T>(&'a mut Stack<T>);

impl<T> Stack<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, item: T) -> Guard<'_, T> {
        self.0.push(item);
        Guard(self)
    }

    fn pop(&mut self) {
        drop(self);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = Stack<T>;
    fn deref(&self) -> &Stack<T> {
        self.0
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

fn main() {
    let mut stack = Stack::new();
    {
        let mut guard = stack.push(1);
        println!("{:?}", guard); // [1]
        {
            let guard2 = guard.push(2);
            println!("{:?}", guard2); // [1, 2]
        }
        {
            let guard3 = guard.push(3);
            println!("{:?}", guard3); // [1, 3]
        }
    }

    let mut my_stack = Stack::new();
    let mut g = my_stack.push(0);
    println!("{:?}", g);

    g.push(2);
    println!("{:?}", g);

    g.pop();
    println!("{:?}", g);

    // assert_eq!(stack.0[..], []);

    // --- --- ---

    // let mut stack = Vec::new();

    // println!("{:?}", stack);

    // stack.push(1);
    // println!("{:?}", stack);

    // stack.push(3);
    // println!("{:?}", stack);

    // stack.push(5);
    // println!("{:?}", stack);

    // stack.pop();
    // println!("{:?}", stack);

    // stack.pop();
    // println!("{:?}", stack);

    // stack.pop();
    // println!("{:?}", stack);

    // let first = stack.pop().unwrap();
    // println!("first:{}", first);
}
