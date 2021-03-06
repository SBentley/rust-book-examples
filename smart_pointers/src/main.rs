use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

mod interior_mutability;
mod tree;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

//    let list = Cons(1,Box::new(Cons(2,Box::new( Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5, *y);
    
    let y = Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
/*
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

*/
    // Combine RefCell and Rc
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    tree::tree();
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}
