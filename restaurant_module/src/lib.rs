
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }

        // expose a private variable on the struct.
        pub fn seasonal_fruit(&self) -> &String {
            &self.seasonal_fruit
        }
    }
}

// The lib .rs structure is an allegory to the file structure within the module

// crate (restaurant_module)
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment


// We can access elements of a module via relative or absolute paths
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn order_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please, the fruit is: {}", meal.toast, meal.seasonal_fruit());
}