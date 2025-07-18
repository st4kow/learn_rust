mod front_of_house { // This is a new modle
    pub mod hosting { // This is a submodule
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving { // This is an other submodule
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,      // This is public
        seasonal_fruit: String, // This is private
    }
    
    impl Breakfast { //Constructor
        pub fn summer(toast: &str) -> Breakfast  {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    
    pub enum Appetizer { // Public enum
        Soup, // Public automatically!
        Salad
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // This refers to one hierarchy higher
    }  
    fn cook_order() {}
}
mod customer {
    pub fn eat_at_restaurant() {
        //hosting::add_to_waitlist(); // This does not work, because hosting was not brought to scope
        super::hosting::add_to_waitlist(); // This works because module above brought hosting into scope
        // OR we could use the "use" keyword in this module to bring hosting into scope
    }
}

//use crate::front_of_house::hosting; // This brings hosting into scope
use crate::front_of_house::hosting::add_to_waitlist; // This brings only the function itself to scope
                                                     // Not used most of the time. Makes it hard to decide 
                                                     // Where the function comes.
// This is used mostly when we bring types into scope
// For example structs or enums
use std::collections::HashMap;
pub fn do_smthg_with_hasmap() {
    let mut map = HashMap::new();
    map.insert(1,2);
}

// If two items with the same name are brought to scope, compiler gives an error.
// In this case this cannot be done, and only the module need to be "use"d
// Or ned to use a different name by "as"

/* RE exporting */
pub use crate::front_of_house::hosting; //TODO it does not work for some reason
// This brings hosting into scope for every module undert this root module
// Also affects documentation.

//Using nested paths for use list
//  use std::cmp::Ordering;
//  use std::io;
//Can be written as
//  use std::{cmp::Ordering, io};

//  use std::io;
//  use std::io::Write;
//Can be writte nas
use std::io::{self, Write};

// Glob operator
use std::collections::*;
// Mosty for collections

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    //Relative path
    front_of_house::hosting::add_to_waitlist();
    
    //Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change out mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // This line will not compile, becasue seasonal_fruit is not public
    //meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup; 
    let order2 = back_of_house::Appetizer::Salad;
    //These are working, because public Enums have public data
    // Reason: Enums are only useful if the data is public

    //Playing with scopes
    hosting::add_to_waitlist(); //This works, because hosting was brought into scope
    add_to_waitlist(); // Works, because this function was brought to scope

}
