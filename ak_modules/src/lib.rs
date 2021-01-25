// pub to expose everything (mod/struct/enum/fn/etc)
// front_of_house::hosting::add_to_waitlist()
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // use serve_order at top of this module
        super::serve_order();
    }
    fn cook_order() {}

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

// htis puts hosting into scope
// generally, use the parent module to keeop it all together instead of directly using the function
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // I don;t need full path, because I have use above
    hosting::add_to_waitlist();
}

pub fn serve_order() {}