fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    }
    // so like this it doesnt actualy do anyting, we need to consume it
    v1.iter().map(|x| x + 1);

    // like this and we dont need to set what the returning type of it is
    let s: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let counter = Counter::new();
    for c in counter {
        println!("{}", c);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // we have to use into iter here because we need to take ownership of that shoe
    // so that we can return it as a new vector
    return shoes.into_iter().filter(|x| x.size == shoe_size).collect();
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        return Counter { count: 0 };
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        };
    }
}

#[cfg(test)]
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    // 1 2 3 4 5
    // 2 3 4 5 None

    // 2 6 12 20 x
    // x T T x x
    // 18

    assert_eq!(18, sum);
}
