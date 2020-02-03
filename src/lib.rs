// front of house module with other modules that in turn contain functions
// this lib.rs module is further contained in an implicit module known as a root crate
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative Path
    front_of_house::hosting::add_to_waitlist()
}
