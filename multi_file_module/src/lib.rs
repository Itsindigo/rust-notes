// require front_of_house module (searches front_of_house directory).
mod front_of_house;

// re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
