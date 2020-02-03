// front of house module with other modules that in turn contain functions
// this lib.rs module is further contained in an implicit module known as a root crate
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
