use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(25, 2);

    let x = vec![1, 2, 3, 4];
    let equal_to_x = move |z| z == x;

    // println!("can't use x here {:?}", x);
    // since the closure got the ownership of x
    // with the command move

    let y = vec![1, 2, 3, 4];
    assert!(equal_to_x(y));
}

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Clone + Copy,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Clone + Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        return Cacher {
            calculation,
            value: HashMap::new(),
        };
    }

    fn value(&mut self, arg: K) -> V {
        return match self.value.contains_key(&arg) {
            true => self.value.get(&arg).unwrap().clone(),
            false => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        };
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
