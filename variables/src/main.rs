fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1); //types are optional here
    let (x, y, z) = tup;
    // we can get values out of tuples from destructuring
    println!("The value of x, y, z is {} {} {}", x, y, z);

    // or like index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // declaring this type is optional
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // this creates an array with 5 3s
    let a = [3; 5];
    let x = 2;
    a_function(x);
}

fn a_function(mut x: i32) {
    x += 2;
    println!("The value of x is: {}", x);
}
