fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v2.push(5);

    let third = v2[2]; // this can cause a panic
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(v) => println!("The third element is {}", v),
        None => println!("There is no third element"),
    }

    // non mutable for loop
    for i in &v2 {
        println!("{}", i);
    }

    // mutable for loop
    for i in &mut v2 {
        *i += 50;
    }
}
