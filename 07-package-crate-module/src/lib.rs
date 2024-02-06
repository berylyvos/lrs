mod front_of_house;

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    back_of_house::fix_incorrect_order();

    hosting::add_to_waitlist();
}

fn server_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        super::server_order();
        crate::server_order();
    }
}