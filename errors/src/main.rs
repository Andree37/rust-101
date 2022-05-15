extern crate core;

use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let arr = vec![1, 2, 3, 4];
    let b = arr.get(100);
    match b {
        None => println!("yo bro this broke"),
        Some(v) => println!("{}", v),
    }

    let f = File::open("hello.txt");

    match f {
        Ok(file) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => println!("File was created"),
                Err(e) => println!("Something went wrong {}", e),
            },
            other => println!("Problem opening the file {:?}", other),
        },
    }

    // instead of the above, we could do:
    let f = File::open("another.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("another.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file {:?}", error))
        } else {
            panic!("Problem opening the file {:?}", error)
        }
    });

    // even simpler with unwrap we could do something like:
    // you have to assume here it gonna be the OK on the match
    // if its not, then it will panic
    let f = File::open("yetanother.txt").unwrap();

    // similar thing with the expect, but you can choose a message
    let f = File::open("anotheranother.txt").expect("Problem opening the file");

    let a = read_username_from_file();

    let b = propagate();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("players.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn propagate() -> Result<String, io::Error> {
    // SAME THING AS ABOVE, BUT NOW we are using macros for sending the error up if it ever exists
    let mut f = File::open("anotheronethatdoesntexist.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn option() -> Option<String> {
    let a = vec![String::from("a"), String::from("b"), String::from("c")];
    return Some(a.get(100)?.to_owned());
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
