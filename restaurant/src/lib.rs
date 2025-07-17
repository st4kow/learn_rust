mod front_of_house { // This is a new modle
    mod hosting { // This is a submodule
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving { // This is an other submodule
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
