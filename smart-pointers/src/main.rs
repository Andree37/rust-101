// Box<T> for allocating values on the Heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T> a type that enforces the borrowing
// rules at runtime instead of compile time.

use crate::List::{Cons, Nil};
use std::ops::Deref;

// Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Our own 'Box' type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

// Drop
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Customer with data {}", &self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b: {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // we can defer like we do with normal references
    assert_eq!(5, *y);

    let x1 = 5;
    let y1 = MyBox::new(x);

    assert_eq!(5, x1);
    // this gives an error if we don't implement the Deref trait
    assert_eq!(5, *y1);

    // See the deref coercion in action:
    let m = MyBox::new(String::from("Andre"));
    hello(&m); // param of hello is &str which is a string vector, not a &String. Yet it works :)

    // Drop
    let c = CustomSmartPointer {
        data: String::from("potato"),
    };
    let d = CustomSmartPointer {
        data: String::from("other potato"),
    };
    println!("SmartCustomPointers created");
}
