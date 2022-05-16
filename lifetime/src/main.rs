use std::fmt::{Debug, Display};

fn main() {
    let number_list = vec![1, 2, 3, 4, 5];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number is {}", largest);

    let r = longest("abcd", "ab");
    println!("{}", r);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut lar = &list[0];

    for item in list {
        if item > lar {
            lar = item;
        }
    }

    return lar;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Summary for Point<T> {
    fn summarize(&self) -> String {
        return format!("potato");
    }
}

impl<T> Point<T> {
    fn getX(&self) -> &T {
        return &self.x;
    }
}

pub trait Summary {
    // it behaves like a generic class
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        return String::from("I am the author");
    }
}

//we can also make sure that parameters have traits
fn notify(item: &(impl Summary + Display)) {
    println!("Summary: {}", item.summarize());
}

// we can make it a bit prettier with
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}

// lifetime
// this 'a ties both lifetimes to the same lifetime as the one that has the longest lifetime
// this is useful whenever we don't know which we will have in the end
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() > y.len() { x } else { y };
}
