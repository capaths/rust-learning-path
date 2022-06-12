use crate::garden::vegetables::Asparagus; // import the Asparagus struct from the vegetables module
use std::io::Result as IoResult; // importing with a different name

mod front_of_house;
use crate::front_of_house::hosting;

use rand::Rng;

pub mod garden; // declare the garden public module

// declares inline module
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;


mod customer {
    pub fn _eat_at_restaurant() {
        // hosting::add_to_waitlist(); -> error, the use of hosting is not in scope
    }
}


// pub use crate::front_of_house::hosting; // exposes the hosting module -> re-exporting


pub fn eat_at_restaurant() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // the use shorthand by module
    hosting::add_to_waitlist();

    // the use shorthand by item (not recommended)
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Order a breakfast in the summer with Rye toast

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn grow_asparagus() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}


fn rand() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}


/*
use std::cmp::Ordering;
use std::io;

==EQUIVALENT TO==
use std::{cmp::Ordering, io};
 */


 /*
 use std::io;
use std::io::Write;
==EQUIVALENT TO==
use std::{self, Write};
*/

/*
use std::collections::*;

to just bring everything
*/