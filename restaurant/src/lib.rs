mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
        pub fn do_wait() {
            crate::front_of_house::serving::wait();
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        pub fn wait() {
            take_order();
            serve_order();
            take_payment();
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::seat_at_table();

    crate::front_of_house::hosting::do_wait();
}
