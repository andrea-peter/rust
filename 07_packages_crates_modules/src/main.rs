// Declares module implemented in separate file
mod front_of_house;

mod back_of_house {
    pub mod kitchen {
        pub fn cook() {
            println!("Cooked");
        }
    }
}

// Patch "shortcuts"
use back_of_house::kitchen;
use back_of_house::kitchen::cook;

fn main() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    kitchen::cook();
    cook();
}
