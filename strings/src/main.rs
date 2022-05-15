use std::collections::HashMap;

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

    // hash maps
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("red"), 50);
    scores.insert(String::from("blue"), 20);

    let teams = vec![
        String::from("Blue"),
        String::from("Red"),
        String::from("Yellow"),
    ];
    let initial_scores = vec![10, 50, 20];

    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // use to upsert
    let value = scores.entry(String::from("Other")).or_insert(10);
    // and we can use the value and do *value += 1 for instance to update the value
}
