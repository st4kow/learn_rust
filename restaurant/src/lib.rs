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
}
