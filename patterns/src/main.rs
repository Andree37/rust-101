struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite colors, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y) = (1, 2);
    let point = (x, y);
    print_coordinates(&point);

    let x = 1;

    match x {
        1 | 2 | 3..=5 => println!("one or two or 3 to 5"),
        //    'a'..='z' => println!("got letter thingie"),
        _ => println!("anything"),
    }

    let p = Point { x: 2, y: 5 };

    let Point { x, y } = p;
    // get x and y from point struct
    assert_eq!(x, 2);
    assert_eq!(y, 5);

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            // the @ lets us use the actual variable and not a new one in the scope
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current Location: ({} {})", x, y);
}
