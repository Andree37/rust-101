fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let ss = data.to_string();

    // appending to a string with push_str and push
    s.push_str(&data);

    let mut lol = String::from("lo");
    lol.push('l');

    // contact with !format or +
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("World!");
    let s3 = s1 + &s2; // we can no longer use s1 after this as s1 is actually changed

    s2 = String::from("potato"); // even though we send a reference to s2, changing it wont change s3

    println!("{}", s3);

    let s3 = format!("{} {}", s3, s2); // format does not take ownership of any of the strings

    // access individual chars of a string

    let s = String::from("Hello");
    // let h = s[0]; this wont compile as string does not have an index implemented

    // we should not index strings in rust, instead do this
    for c in s.chars() {
        println!("{}", c);
    }
}
