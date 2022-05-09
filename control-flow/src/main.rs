fn main() {
    println!(
        "32 Fahrenheit to Celsius is {}",
        convert_temperature(32, "C")
    );

    println!(
        "32 Celsius to Fahrenheit is {}",
        convert_temperature(32, "F")
    );

    println!("Fibonnaci number of {} is {}", 5, fibonnaci(5));
    println!("Fibonnaci number of {} is {}", 10, fibonnaci(10));
    println!("Fibonnaci number of {} is {}", 2, fibonnaci(2));
    println!("Fibonnaci number of {} is {}", 0, fibonnaci(0));
    println!("Fibonnaci number of {} is {}", -1, fibonnaci(-1));

    println!("{}", twelve_days_of_xmas());
}

fn convert_temperature(t: i32, to: &str) -> i32 {
    match to {
        "F" => return (t * 9 / 5) + 32,
        "C" => return (t - 32) * 5 / 9,
        _ => return 0,
    }
}

fn fibonnaci(i: i32) -> i32 {
    if i < 2 {
        return i;
    }

    return fibonnaci(i - 1) + i;
}

fn twelve_days_of_xmas() -> String {
    let mut result = "".to_owned();
    let mut caller = "".to_owned();
    let gives = [
        "A song for the Christmas tree",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];

    for i in 0..12 {
        caller = format!("{}\n{}", gives[i], caller);
        result = format!(
            "{}\nOn the {} day of christmas, santa gave to me \n{}",
            result,
            i + 1,
            caller
        )
    }

    return result.to_owned();
}
