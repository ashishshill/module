// module can take almost everything like enum
use rand::CryptoRng;
use rand::ErrorKind::Transient;
use rand::Rng;
// using nested path for  better code.
use std::io::{self, Write};

mod front_of_hous {
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
//
mod front_of_house;

pub use self::front_of_house::hosting; //this for using create scope

pub fn eat_at_restaurant() {
    let number = rand::thread_rng().gen_range(1, 101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// this code are not working beacause rust cannot see the child and child also cannot see the praents also.
// if we are using pub then it will be okay
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // here we can also use super keyword to allow as paraent module.
}
