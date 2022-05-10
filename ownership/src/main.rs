fn something() {
    let mut s = String::from("hello");
    s.push_str(", world"); //push_str append a literal to a String

    // we can use s here as the owner of it is in scope

    // assigning variables to others are like c
    // it copies the pointer to it
    // unless it is a basic type like an int or float, etc

    let s2 = s; // this is considered a 'move'; s was moved to s2
                //println!("{}", s); // this throws an error since s is considered invalid

    // a way to create a whole other string is:
    let s3 = s2.clone(); // this copies everything to another variable on the heap

    // passing a value to a function will move or copy depending on the variable type
}

// we can no longer use s as the owner of it is out of scope

fn other() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// ===== REFERENCES AND BORROWING =======

// we can pass variables as references only, not letting them actually mutate it
// this allows for the case where s would get out of scope
// when we are really just accessing its value and not changing it

fn oother() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // we can still use s1 as it was only borrowed
    // and not actually moved
}
fn calculate_length(s: &String) -> usize {
    return s.len();
} // since the function doesnt own s, it doesnt drop it

// we can have mutable references

fn ooother() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s); // i can still use it even though it has been sent to another function and it can modify its value
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello world");
    let first = first_word(&s);

    // this results in an error
    // as s is already being borrowed, it can not be mutated
    // until first gets out of scope
    // s = String::from("");

    // since we are using first here, we cannot mutate it
    // if we used it above the commented line, it would work
    println!("{}", first);
}

fn first_word(s: &str) -> &str {
    // using string slices
    // it points to portions of the string
    // and whenever the string get updated, so will the value from these pointers

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return s;
}
