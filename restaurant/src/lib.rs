mod front_of_the_house {
    pub mod hosting {
        pub(crate) fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    fn fix_incorrect_order() {        
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
mod a {
    pub(super) fn in_a() {}

    mod b {
        pub(super) fn in_b() {}

        mod c {
            pub(super) fn call_parents() {
                // Call in_a and in_b from here using super
                super::in_b();
                super::super::in_a();
            }
        }
    }
}

fn deliver_order(){}
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // Relative path
    front_of_the_house::hosting::add_to_waitlist();
}