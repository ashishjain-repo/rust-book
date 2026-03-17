use std::{cmp::Ordering, io};
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
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
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
use crate::front_of_house::hosting;
pub fn eat_at_resaurant() {
    /* crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist(); */
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
    let order1 = back_of_house::Appetizer::Soup;
}

mod customer {
    use crate::front_of_house::hosting;
    use crate::front_of_house::hosting::add_to_waitlist;
    use crate::front_of_house::hosting::add_to_waitlist as waitlist;
    pub fn eat_at_resaurant() {
    hosting::add_to_waitlist();
    add_to_waitlist();
    waitlist();
    }
}