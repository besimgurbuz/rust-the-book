mod front_of_house;
// use crate::front_of_house::hosting;
use std::io::Result as IoResult;

// re-exporting names with pub use
pub use crate::front_of_house::{hosting, total_customer_size};
// use crate::front_of_house::*;

// the front_of_house isn't public, but because the eat_at_restaurant function defined in the same module as front_of_house, we can refer to front_of_house
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
    // after `use` keyword, we can just use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("currently {} people are dining", total_customer_size());
}

// we can also construct relative paths that begin in the parent module by using super at the start of the path. this is like starting a filesystem path with the `..` syntax.
// for example;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
    // Making structs and enums public
    // We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub before a struct definition, we make the struct public,
    // but the struct's fields will still be private.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        // Becase back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast (summer)
        // if Breakfast didn't have such a function, we couldn't create an instance of Breakfast in eat_at_restaurant because we couldn't set the value of the private
        // seasonal_fruit field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If we make an enum public, all of its variants are then public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
