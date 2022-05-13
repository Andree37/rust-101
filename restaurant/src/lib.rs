mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                return Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                };
            }
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn fix_incorrect_order() {
            take_order();
            // we can use super:: to refer to the ../
            super::serve_order();
        }
    }

    fn serve_order() {}

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        // we cannot access seasonal_fruit as it is private
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
