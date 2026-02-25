pub mod auth;
pub mod collections_;
pub mod errors;
pub mod garden;
mod guessing_game;
pub mod library;
pub mod utils;
pub mod f_ {
    pub mod host {
        pub fn add_to_waitlist() {}
    }
}

use crate::library::tsc::tsc_utils::test as tsc_test;

pub fn tsc_utils_test() {
    tsc_test();
}

use crate::f_::host;

pub fn eat_at_restaurant_() {
    host::add_to_waitlist();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn greet() {
    println!("--- Assalamu alaykum ---");
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod customer {
    // use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist(); // A use statement only applies in the scope it’s in
        super::hosting::add_to_waitlist(); // or via using super
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

// mod back_of_house {
//     pub enum Appetizer { // fully public enum
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }
