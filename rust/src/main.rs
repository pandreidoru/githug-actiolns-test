enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        main();

        for i in 1..=4 {
            println!("{}", i);
        }
    }
}